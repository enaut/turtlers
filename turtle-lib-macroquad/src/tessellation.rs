//! Lyon tessellation utilities for turtle graphics
//!
//! This module provides helper functions to tessellate paths using Lyon,
//! which replaces the manual triangulation with GPU-optimized tessellation.

use crate::state::MeshData;
use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;
use macroquad::prelude::*;

/// Convert macroquad Vec2 to Lyon Point
pub fn to_lyon_point(v: Vec2) -> Point {
    point(v.x, v.y)
}

/// Convert Lyon Point to macroquad Vec2
#[allow(dead_code)]
pub fn to_macroquad_vec2(p: Point) -> Vec2 {
    vec2(p.x, p.y)
}

/// Simple vertex type for Lyon tessellation
#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
    pub position: [f32; 2],
}

/// Build mesh data from Lyon tessellation
pub fn build_mesh_data(vertices: &[SimpleVertex], indices: &[u16], color: Color) -> MeshData {
    let verts: Vec<Vertex> = vertices
        .iter()
        .map(|v| Vertex {
            position: Vec3::new(v.position[0], v.position[1], 0.0),
            uv: Vec2::ZERO,
            color: [
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                (color.a * 255.0) as u8,
            ],
            normal: Vec4::ZERO,
        })
        .collect();

    MeshData {
        vertices: verts,
        indices: indices.to_vec(),
    }
}

/// Tessellate a polygon and return mesh
///
/// This automatically handles holes when the path crosses itself.
pub fn tessellate_polygon(
    vertices: &[Vec2],
    color: Color,
) -> Result<MeshData, Box<dyn std::error::Error>> {
    if vertices.is_empty() {
        return Err("No vertices provided".into());
    }

    // Build path
    let mut builder = Path::builder();
    builder.begin(to_lyon_point(vertices[0]));
    for v in &vertices[1..] {
        builder.line_to(to_lyon_point(*v));
    }
    builder.end(true); // Close the path

    let path = builder.build();

    // Tessellate with EvenOdd fill rule (automatic hole detection)
    let mut geometry: VertexBuffers<SimpleVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    tessellator.tessellate_path(
        &path,
        &FillOptions::default().with_fill_rule(FillRule::EvenOdd),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| SimpleVertex {
            position: vertex.position().to_array(),
        }),
    )?;

    Ok(build_mesh_data(
        &geometry.vertices,
        &geometry.indices,
        color,
    ))
}

/// Tessellate multiple contours (outer boundary + holes) and return mesh
///
/// The first contour is the outer boundary, subsequent contours are holes.
/// Lyon's EvenOdd fill rule automatically creates holes where contours overlap.
pub fn tessellate_multi_contour(
    contours: &[Vec<Vec2>],
    color: Color,
) -> Result<MeshData, Box<dyn std::error::Error>> {
    if contours.is_empty() {
        return Err("No contours provided".into());
    }

    eprintln!("\n=== tessellate_multi_contour Debug ===");
    eprintln!("Total contours to tessellate: {}", contours.len());

    // Build path with multiple sub-paths (contours)
    let mut builder = Path::builder();

    for (idx, contour) in contours.iter().enumerate() {
        if contour.is_empty() {
            eprintln!("WARNING: Contour {} is empty, skipping", idx);
            continue;
        }

        eprintln!("\nContour {}: {} vertices", idx, contour.len());
        eprintln!("  First vertex: ({:.2}, {:.2})", contour[0].x, contour[0].y);
        if contour.len() > 1 {
            eprintln!(
                "  Last vertex:  ({:.2}, {:.2})",
                contour[contour.len() - 1].x,
                contour[contour.len() - 1].y
            );
        }

        // Each contour is a separate closed sub-path
        builder.begin(to_lyon_point(contour[0]));
        for (i, v) in contour[1..].iter().enumerate() {
            builder.line_to(to_lyon_point(*v));
            if i < 3 || i >= contour.len() - 4 {
                eprintln!("    Vertex {}: ({:.2}, {:.2})", i + 1, v.x, v.y);
            } else if i == 3 {
                eprintln!("    ... ({} more vertices)", contour.len() - 7);
            }
        }
        builder.end(true); // Close this contour
        eprintln!("  Contour closed");
    }

    eprintln!("\nBuilding Lyon path...");
    let path = builder.build();
    eprintln!("Path built successfully");

    // Tessellate with EvenOdd fill rule - overlapping areas become holes
    let mut geometry: VertexBuffers<SimpleVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    eprintln!("Starting tessellation with EvenOdd fill rule...");
    match tessellator.tessellate_path(
        &path,
        &FillOptions::default().with_fill_rule(FillRule::EvenOdd),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| SimpleVertex {
            position: vertex.position().to_array(),
        }),
    ) {
        Ok(_) => {
            eprintln!("Tessellation successful!");
            eprintln!(
                "  Generated {} vertices, {} indices",
                geometry.vertices.len(),
                geometry.indices.len()
            );
            eprintln!("  Triangles: {}", geometry.indices.len() / 3);
        }
        Err(e) => {
            eprintln!("ERROR: Tessellation failed: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(build_mesh_data(
        &geometry.vertices,
        &geometry.indices,
        color,
    ))
}

/// Tessellate a stroked path and return mesh
pub fn tessellate_stroke(
    vertices: &[Vec2],
    color: Color,
    width: f32,
    closed: bool,
) -> Result<MeshData, Box<dyn std::error::Error>> {
    if vertices.is_empty() {
        return Err("No vertices provided".into());
    }

    // Build path
    let mut builder = Path::builder();
    builder.begin(to_lyon_point(vertices[0]));
    for v in &vertices[1..] {
        builder.line_to(to_lyon_point(*v));
    }
    builder.end(closed);
    let path = builder.build();

    // Tessellate with round caps and joins for smooth lines
    let mut geometry: VertexBuffers<SimpleVertex, u16> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    tessellator.tessellate_path(
        &path,
        &StrokeOptions::default()
            .with_line_width(width)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
        &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| SimpleVertex {
            position: vertex.position().to_array(),
        }),
    )?;

    Ok(build_mesh_data(
        &geometry.vertices,
        &geometry.indices,
        color,
    ))
}

/// Tessellate a circle and return mesh
pub fn tessellate_circle(
    center: Vec2,
    radius: f32,
    color: Color,
    filled: bool,
    stroke_width: f32,
) -> Result<MeshData, Box<dyn std::error::Error>> {
    let mut builder = Path::builder();
    builder.add_circle(to_lyon_point(center), radius, lyon::path::Winding::Positive);
    let path = builder.build();

    let mut geometry: VertexBuffers<SimpleVertex, u16> = VertexBuffers::new();

    if filled {
        let mut tessellator = FillTessellator::new();
        tessellator.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| SimpleVertex {
                position: vertex.position().to_array(),
            }),
        )?;
    } else {
        let mut tessellator = StrokeTessellator::new();
        tessellator.tessellate_path(
            &path,
            &StrokeOptions::default().with_line_width(stroke_width),
            &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| SimpleVertex {
                position: vertex.position().to_array(),
            }),
        )?;
    }

    Ok(build_mesh_data(
        &geometry.vertices,
        &geometry.indices,
        color,
    ))
}

/// Tessellate an arc (partial circle) and return mesh
pub fn tessellate_arc(
    center: Vec2,
    radius: f32,
    start_angle_degrees: f32,
    arc_angle_degrees: f32,
    color: Color,
    stroke_width: f32,
    segments: usize,
) -> Result<MeshData, Box<dyn std::error::Error>> {
    // Build arc path manually from segments
    let mut builder = Path::builder();

    let start_angle = start_angle_degrees.to_radians();
    let arc_angle = arc_angle_degrees.to_radians();
    let step = arc_angle / segments as f32;

    // Calculate first point
    let first_angle = start_angle;
    let first_point = point(
        center.x + radius * first_angle.cos(),
        center.y + radius * first_angle.sin(),
    );
    builder.begin(first_point);

    // Add remaining points
    for i in 1..=segments {
        let angle = start_angle + step * i as f32;
        let pt = point(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        );
        builder.line_to(pt);
    }

    builder.end(false); // Don't close the arc
    let path = builder.build();

    // Tessellate stroke
    let mut geometry: VertexBuffers<SimpleVertex, u16> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    tessellator.tessellate_path(
        &path,
        &StrokeOptions::default()
            .with_line_width(stroke_width)
            .with_line_cap(lyon::tessellation::LineCap::Round)
            .with_line_join(lyon::tessellation::LineJoin::Round),
        &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| SimpleVertex {
            position: vertex.position().to_array(),
        }),
    )?;

    Ok(build_mesh_data(
        &geometry.vertices,
        &geometry.indices,
        color,
    ))
}
