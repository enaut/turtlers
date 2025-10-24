//! SVG-Export-Backend für TurtleWorld

#[cfg(feature = "svg")]
pub mod svg_export {
    use crate::commands::TurtleCommand;
    use crate::export::{DrawingExporter, ExportError};
    use crate::state::{DrawCommand, TurtleWorld};
    use std::fs::File;
    use svg::{
        node::element::{Circle, Line, Polygon, Text as SvgText},
        Document,
    };

    pub struct SvgExporter;

    impl DrawingExporter for SvgExporter {
        fn export(&self, world: &TurtleWorld, filename: &str) -> Result<(), ExportError> {
            let mut doc = Document::new();

            let mut min_x = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut min_y = f32::INFINITY;
            let mut max_y = f32::NEG_INFINITY;

            fn update_bounds(
                min_x: &mut f32,
                max_x: &mut f32,
                min_y: &mut f32,
                max_y: &mut f32,
                x: f32,
                y: f32,
            ) {
                *min_x = min_x.min(x);
                *max_x = max_x.max(x);
                *min_y = min_y.min(y);
                *max_y = max_y.max(y);
            }

            for turtle in &world.turtles {
                for cmd in &turtle.commands {
                    match cmd {
                        DrawCommand::Mesh { source, .. } => {
                            match &source.command {
                                TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
                                    // Linie als <line>
                                    let start = source.start_position;
                                    let end = source.end_position;
                                    update_bounds(
                                        &mut min_x, &mut max_x, &mut min_y, &mut max_y, start.x,
                                        start.y,
                                    );
                                    update_bounds(
                                        &mut min_x, &mut max_x, &mut min_y, &mut max_y, end.x,
                                        end.y,
                                    );
                                    let line = Line::new()
                                        .set("x1", start.x)
                                        .set("y1", start.y)
                                        .set("x2", end.x)
                                        .set("y2", end.y)
                                        .set("stroke", color_to_svg(source.color))
                                        .set("stroke-width", source.pen_width);
                                    doc = doc.add(line);
                                }
                                TurtleCommand::Circle {
                                    radius,
                                    angle,
                                    direction,
                                    ..
                                } => {
                                    use crate::circle_geometry::CircleGeometry;
                                    let geom = CircleGeometry::new(
                                        source.start_position,
                                        source.start_heading,
                                        *radius,
                                        *direction,
                                    );
                                    let center = geom.center;
                                    if (*angle - 360.0).abs() < 1e-3 {
                                        // Voller Kreis
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            center.x - radius,
                                            center.y - radius,
                                        );
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            center.x + radius,
                                            center.y + radius,
                                        );
                                        let circle = Circle::new()
                                            .set("cx", center.x)
                                            .set("cy", center.y)
                                            .set("r", *radius)
                                            .set("stroke", color_to_svg(source.color))
                                            .set("stroke-width", source.pen_width)
                                            .set("fill", "none");
                                        doc = doc.add(circle);
                                    } else {
                                        // Kreisbogen als <path>
                                        let start = source.start_position;
                                        let end = source.end_position;
                                        // For arcs, include the full circle bounds to ensure complete visibility
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            center.x - radius,
                                            center.y - radius,
                                        );
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            center.x + radius,
                                            center.y + radius,
                                        );
                                        let large_arc = if *angle > 180.0 { 1 } else { 0 };
                                        let sweep = match direction {
                                            crate::circle_geometry::CircleDirection::Left => 0,
                                            crate::circle_geometry::CircleDirection::Right => 1,
                                        };
                                        let d = format!(
                                            "M {} {} A {} {} 0 {} {} {} {}",
                                            start.x,
                                            start.y,
                                            radius,
                                            radius,
                                            large_arc,
                                            sweep,
                                            end.x,
                                            end.y
                                        );
                                        let path = svg::node::element::Path::new()
                                            .set("d", d)
                                            .set("stroke", color_to_svg(source.color))
                                            .set("stroke-width", source.pen_width)
                                            .set("fill", "none");
                                        doc = doc.add(path);
                                    }
                                }
                                TurtleCommand::EndFill => {
                                    // Fills werden als <path> mit Konturen ausgegeben
                                    if let Some(contours) = &source.contours {
                                        for contour in contours {
                                            for point in contour {
                                                update_bounds(
                                                    &mut min_x, &mut max_x, &mut min_y, &mut max_y,
                                                    point.x, point.y,
                                                );
                                            }
                                        }
                                        let mut d = String::new();
                                        for (i, contour) in contours.iter().enumerate() {
                                            if !contour.is_empty() {
                                                if i > 0 {
                                                    d.push(' ');
                                                }
                                                d.push_str(&format!(
                                                    "M {} {}",
                                                    contour[0].x, contour[0].y
                                                ));
                                                for point in contour.iter().skip(1) {
                                                    d.push_str(&format!(
                                                        " L {} {}",
                                                        point.x, point.y
                                                    ));
                                                }
                                                d.push_str(" Z");
                                            }
                                        }
                                        if !d.is_empty() {
                                            let path = svg::node::element::Path::new()
                                                .set("d", d)
                                                .set("fill", color_to_svg(source.fill_color))
                                                .set("fill-rule", "evenodd")
                                                .set("stroke", color_to_svg(source.color));
                                            doc = doc.add(path);
                                        }
                                    } else {
                                        // Fallback: Dummy-Polygon
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            source.start_position.x,
                                            source.start_position.y,
                                        );
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            source.start_position.x + 10.0,
                                            source.start_position.y + 10.0,
                                        );
                                        update_bounds(
                                            &mut min_x,
                                            &mut max_x,
                                            &mut min_y,
                                            &mut max_y,
                                            source.start_position.x + 5.0,
                                            source.start_position.y + 15.0,
                                        );
                                        let poly = Polygon::new()
                                            .set(
                                                "points",
                                                format!(
                                                    "{},{} {},{} {},{}",
                                                    source.start_position.x,
                                                    source.start_position.y,
                                                    source.start_position.x + 10.0,
                                                    source.start_position.y + 10.0,
                                                    source.start_position.x + 5.0,
                                                    source.start_position.y + 15.0
                                                ),
                                            )
                                            .set("fill", color_to_svg(source.fill_color))
                                            .set("stroke", color_to_svg(source.color));
                                        doc = doc.add(poly);
                                    }
                                }
                                _ => {}
                            }
                        }
                        DrawCommand::Text {
                            text,
                            position,
                            source,
                            ..
                        } => {
                            update_bounds(
                                &mut min_x, &mut max_x, &mut min_y, &mut max_y, position.x,
                                position.y,
                            );
                            let txt = SvgText::new()
                                .set("x", position.x)
                                .set("y", position.y)
                                .set("fill", color_to_svg(source.color))
                                .add(svg::node::Text::new(text.clone()));
                            doc = doc.add(txt);
                        }
                    }
                }
            }

            // Set viewBox with 20px padding
            if min_x.is_finite() && max_x.is_finite() && min_y.is_finite() && max_y.is_finite() {
                let width = (max_x - min_x) + 40.0;
                let height = (max_y - min_y) + 40.0;
                let view_box = format!("{} {} {} {}", min_x - 20.0, min_y - 20.0, width, height);
                doc = doc.set("viewBox", view_box);
            } else {
                // Default viewBox if no elements
                doc = doc.set("viewBox", "0 0 400 400");
            }

            let mut file = File::create(filename).map_err(ExportError::Io)?;
            svg::write(&mut file, &doc).map_err(ExportError::Io)?;
            Ok(())
        }
    }

    fn color_to_svg(color: crate::general::Color) -> String {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        if color.a < 1.0 {
            format!("rgba({},{},{},{})", r, g, b, color.a)
        } else {
            format!("rgb({},{},{})", r, g, b)
        }
    }
}
