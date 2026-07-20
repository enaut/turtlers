//! SVG export backend for `TurtleWorld`.

#[cfg(feature = "svg")]
pub mod svg_export {
    use crate::export::{DrawingExporter, ExportError};
    use crate::state::{SvgRecord, TurtleWorld};
    use std::fs::File;
    use svg::{
        node::element::{Circle, Line, Text as SvgText},
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
                for record in &turtle.svg_log.records {
                    match record {
                        SvgRecord::Line {
                            start,
                            end,
                            color,
                            pen_width,
                        } => {
                            update_bounds(
                                &mut min_x, &mut max_x, &mut min_y, &mut max_y, start.x, start.y,
                            );
                            update_bounds(
                                &mut min_x, &mut max_x, &mut min_y, &mut max_y, end.x, end.y,
                            );
                            let line = Line::new()
                                .set("x1", start.x)
                                .set("y1", start.y)
                                .set("x2", end.x)
                                .set("y2", end.y)
                                .set("stroke", color_to_svg(*color))
                                .set("stroke-width", *pen_width);
                            doc = doc.add(line);
                        }

                        SvgRecord::Arc {
                            start_position,
                            start_heading,
                            radius,
                            angle,
                            direction,
                            color,
                            pen_width,
                        } => {
                            use crate::circle_geometry::CircleGeometry;
                            use crate::general::Radians;
                            let geom = CircleGeometry::new(
                                *start_position,
                                Radians::new(*start_heading),
                                *radius,
                                *direction,
                            );
                            let center = geom.center;
                            // Include the bounding box of the full circle so partial arcs
                            // are never clipped.
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

                            if (angle.value() - 360.0).abs() < 1e-3 {
                                // Full circle — emit as <circle>
                                let circle = Circle::new()
                                    .set("cx", center.x)
                                    .set("cy", center.y)
                                    .set("r", *radius)
                                    .set("stroke", color_to_svg(*color))
                                    .set("stroke-width", *pen_width)
                                    .set("fill", "none");
                                doc = doc.add(circle);
                            } else {
                                // Partial arc — emit as <path A …>
                                let end = geom.position_at_angle(angle.as_radians().value());
                                let large_arc = if angle.value() > 180.0 { 1 } else { 0 };
                                let sweep = match direction {
                                    crate::circle_geometry::CircleDirection::Left => 0,
                                    crate::circle_geometry::CircleDirection::Right => 1,
                                };
                                let d = format!(
                                    "M {} {} A {} {} 0 {} {} {} {}",
                                    start_position.x,
                                    start_position.y,
                                    radius,
                                    radius,
                                    large_arc,
                                    sweep,
                                    end.x,
                                    end.y,
                                );
                                let path = svg::node::element::Path::new()
                                    .set("d", d)
                                    .set("stroke", color_to_svg(*color))
                                    .set("stroke-width", *pen_width)
                                    .set("fill", "none");
                                doc = doc.add(path);
                            }
                        }

                        SvgRecord::Fill {
                            contours,
                            fill_color,
                            stroke_color,
                        } => {
                            for contour in contours {
                                for point in contour {
                                    update_bounds(
                                        &mut min_x, &mut max_x, &mut min_y, &mut max_y, point.x,
                                        point.y,
                                    );
                                }
                            }
                            let mut d = String::new();
                            for (i, contour) in contours.iter().enumerate() {
                                if !contour.is_empty() {
                                    if i > 0 {
                                        d.push(' ');
                                    }
                                    d.push_str(&format!("M {} {}", contour[0].x, contour[0].y));
                                    for point in contour.iter().skip(1) {
                                        d.push_str(&format!(" L {} {}", point.x, point.y));
                                    }
                                    d.push_str(" Z");
                                }
                            }
                            if !d.is_empty() {
                                let path = svg::node::element::Path::new()
                                    .set("d", d)
                                    .set("fill", color_to_svg(*fill_color))
                                    .set("fill-rule", "evenodd")
                                    .set("stroke", color_to_svg(*stroke_color));
                                doc = doc.add(path);
                            }
                        }

                        SvgRecord::Text {
                            text,
                            position,
                            heading,
                            font_size,
                            color,
                            ..
                        } => {
                            // Match the offset/rotation applied by the on-screen renderer
                            // (see draw_text_command in drawing.rs) so the SVG export lines up.
                            let font_size_val = f32::from(font_size.value());
                            let offset_distance = font_size_val / 3.0;
                            let perpendicular_angle = *heading - std::f32::consts::PI / 2.0;
                            let text_x = position.x + offset_distance * perpendicular_angle.cos();
                            let text_y = position.y + offset_distance * perpendicular_angle.sin();
                            update_bounds(
                                &mut min_x, &mut max_x, &mut min_y, &mut max_y, text_x, text_y,
                            );

                            let rotation_deg = heading.to_degrees();
                            let txt = SvgText::new(text.clone())
                                .set("x", text_x)
                                .set("y", text_y)
                                .set("fill", color_to_svg(*color))
                                .set("font-size", font_size_val)
                                .set(
                                    "transform",
                                    format!("rotate({rotation_deg}, {text_x}, {text_y})"),
                                );
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
