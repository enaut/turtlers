//! SVG export backend for `TurtleWorld`.

#[cfg(feature = "svg")]
pub mod svg_export {
    use crate::export::{DrawingExporter, ExportError};
    use crate::state::{SvgRecord, TurtleWorld};
    use std::fmt::Write;
    use std::fs::File;
    use svg::{
        node::element::{Circle, Line, Text as SvgText},
        Document,
    };

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

    pub struct SvgExporter;

    impl SvgExporter {
        /// Generate an SVG [`Document`] from the given [`TurtleWorld`].
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn to_svg_document(world: &TurtleWorld) -> Document {
            let mut doc = Document::new();

            let mut min_x = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut min_y = f32::INFINITY;
            let mut max_y = f32::NEG_INFINITY;

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
                                .set("stroke-width", *pen_width)
                                .set("stroke-linecap", "round");
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
                            let radius_val = radius.abs();
                            // Include the bounding box of the full circle so partial arcs
                            // are never clipped.
                            update_bounds(
                                &mut min_x,
                                &mut max_x,
                                &mut min_y,
                                &mut max_y,
                                center.x - radius_val,
                                center.y - radius_val,
                            );
                            update_bounds(
                                &mut min_x,
                                &mut max_x,
                                &mut min_y,
                                &mut max_y,
                                center.x + radius_val,
                                center.y + radius_val,
                            );

                            if (angle.value().abs() - 360.0).abs() < 1e-3 {
                                // Full circle — emit as <circle>
                                let circle = Circle::new()
                                    .set("cx", center.x)
                                    .set("cy", center.y)
                                    .set("r", radius_val)
                                    .set("stroke", color_to_svg(*color))
                                    .set("stroke-width", *pen_width)
                                    .set("fill", "none");
                                doc = doc.add(circle);
                            } else {
                                // Partial arc — emit as <path A …>
                                let end = geom.position_at_angle(angle.as_radians().value());
                                let large_arc = i32::from(angle.value().abs() > 180.0);
                                let sweep = match (direction, angle.value() >= 0.0) {
                                    (crate::circle_geometry::CircleDirection::Right, true)
                                    | (crate::circle_geometry::CircleDirection::Left, false) => 1,
                                    (crate::circle_geometry::CircleDirection::Left, true)
                                    | (crate::circle_geometry::CircleDirection::Right, false) => 0,
                                };
                                let d = format!(
                                    "M {} {} A {} {} 0 {} {} {} {}",
                                    start_position.x,
                                    start_position.y,
                                    radius_val,
                                    radius_val,
                                    large_arc,
                                    sweep,
                                    end.x,
                                    end.y,
                                );
                                let path = svg::node::element::Path::new()
                                    .set("d", d)
                                    .set("stroke", color_to_svg(*color))
                                    .set("stroke-width", *pen_width)
                                    .set("stroke-linecap", "round")
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
                                    let _ = write!(d, "M {} {}", contour[0].x, contour[0].y);
                                    for point in contour.iter().skip(1) {
                                        let _ = write!(d, " L {} {}", point.x, point.y);
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

            doc
        }
    }

    impl DrawingExporter for SvgExporter {
        fn export(&self, world: &TurtleWorld, filename: &str) -> Result<(), ExportError> {
            let doc = Self::to_svg_document(world);
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
            format!("rgba({r},{g},{b},{})", color.a)
        } else {
            format!("rgb({r},{g},{b})")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::circle_geometry::CircleDirection;
        use crate::general::{Color, Coordinate, Degrees};
        use crate::state::{SvgRecord, Turtle};

        #[test]
        fn test_svg_export_line_has_round_line_caps() {
            let mut world = TurtleWorld::new();
            let mut turtle = Turtle::default();
            turtle.svg_log.push(SvgRecord::Line {
                start: Coordinate::new(0.0, 0.0),
                end: Coordinate::new(100.0, 0.0),
                color: Color::new(0.0, 0.0, 0.0, 1.0),
                pen_width: 2.0,
            });
            world.turtles.push(turtle);

            let doc = SvgExporter::to_svg_document(&world);
            let svg_string = doc.to_string();
            assert!(
                svg_string.contains(r#"stroke-linecap="round""#),
                "SVG export of lines should have round line caps: {svg_string}"
            );
        }

        #[test]
        fn test_svg_export_arc_has_round_line_caps() {
            let mut world = TurtleWorld::new();
            let mut turtle = Turtle::default();
            turtle.svg_log.push(SvgRecord::Arc {
                start_position: Coordinate::new(0.0, 0.0),
                start_heading: 0.0,
                radius: 50.0,
                angle: Degrees::new(90.0),
                direction: CircleDirection::Right,
                color: Color::new(0.0, 0.0, 0.0, 1.0),
                pen_width: 2.0,
            });
            world.turtles.push(turtle);

            let doc = SvgExporter::to_svg_document(&world);
            let svg_string = doc.to_string();
            assert!(
                svg_string.contains(r#"stroke-linecap="round""#),
                "SVG export of partial arcs should have round line caps: {svg_string}"
            );
        }

        #[test]
        fn test_svg_export_arc_negative_angle_sweep() {
            let mut world = TurtleWorld::new();
            let mut turtle = Turtle::default();
            // Circle right with negative angle should sweep counter-clockwise (sweep = 0)
            turtle.svg_log.push(SvgRecord::Arc {
                start_position: Coordinate::new(0.0, 0.0),
                start_heading: 0.0,
                radius: 50.0,
                angle: Degrees::new(-90.0),
                direction: CircleDirection::Right,
                color: Color::new(0.0, 0.0, 0.0, 1.0),
                pen_width: 2.0,
            });
            // Circle left with negative angle should sweep clockwise (sweep = 1)
            turtle.svg_log.push(SvgRecord::Arc {
                start_position: Coordinate::new(100.0, 100.0),
                start_heading: 0.0,
                radius: 50.0,
                angle: Degrees::new(-90.0),
                direction: CircleDirection::Left,
                color: Color::new(0.0, 0.0, 0.0, 1.0),
                pen_width: 2.0,
            });
            world.turtles.push(turtle);

            let doc = SvgExporter::to_svg_document(&world);
            let svg_string = doc.to_string();
            // Right with negative angle: large_arc=0, sweep=0 -> "0 0 0"
            assert!(
                svg_string.contains("A 50 50 0 0 0"),
                "Circle right with negative angle should have sweep=0: {svg_string}"
            );
            // Left with negative angle: large_arc=0, sweep=1 -> "0 0 1"
            assert!(
                svg_string.contains("A 50 50 0 0 1"),
                "Circle left with negative angle should have sweep=1: {svg_string}"
            );
        }
    }
}
