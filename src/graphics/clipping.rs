use super::vertex::ColorVertex;
use super::{Line, Triangle};
use bitflags::bitflags;
use cgmath::Vector4;

bitflags! {
    struct OutCode: u8 {
        const INSIDE = 0b00000000;
        const LEFT   = 0b00000001;
        const RIGHT  = 0b00000010;
        const BOTTOM = 0b00000100;
        const TOP    = 0b00001000;
        const FRONT  = 0b00010000;
        const BACK   = 0b00100000;
    }
}

pub enum ClippedTriangle<T> {
    Empty,
    One(Triangle<T>),
    Two(Triangle<T>, Triangle<T>),
}

/// Computes the position of a 4-dimensional vector in the unit view frustum.
fn compute_outcode(position: Vector4<f32>) -> OutCode {
    let mut code: OutCode = OutCode::INSIDE;

    if position.x < -position.w {
        code |= OutCode::LEFT;
    } else if position.x > position.w {
        code |= OutCode::RIGHT;
    }
    if position.y < -position.w {
        code |= OutCode::BOTTOM;
    } else if position.y > position.w {
        code |= OutCode::TOP;
    }
    if position.z < -position.w {
        code |= OutCode::FRONT;
    } else if position.z > position.w {
        code |= OutCode::BACK;
    }

    code
}

/// Clips the line against a unit view frustum.
pub fn clip_line(line: Line<ColorVertex>) -> Option<Line<ColorVertex>> {
    let Line(mut e0, mut e1) = line;

    // Calculate where the endpoints are in relation to the clipping rectangle.
    let mut code0 = compute_outcode(e0.position);
    let mut code1 = compute_outcode(e1.position);

    loop {
        if (code0 | code1) == OutCode::INSIDE {
            // Both endpoints are inside the clip region. Trivial accept.
            break Some(Line(e0, e1));
        } else if (code0 & code1) != OutCode::INSIDE {
            // Both endpoints share an outside region. In other words,
            // the line is outside. Trivial reject.
            break None;
        } else {
            // Calculate the segment from an outside point to an intersection to clip.

            // Pick one of the outside segments.
            let code_out = OutCode::max(code0, code1);

            // Find intersection point.
            let delta = e1 - e0;
            let alpha = if code_out.contains(OutCode::RIGHT) {
                // Point is right to the clipping region.
                (e0.position.w - e0.position.x) / (delta.position.x - delta.position.w)
            } else if code_out.contains(OutCode::LEFT) {
                // Point is left to the clipping region.
                (-e0.position.w - e0.position.x) / (delta.position.x + delta.position.w)
            } else if code_out.contains(OutCode::TOP) {
                // Point is above the clipping region.
                (e0.position.w - e0.position.y) / (delta.position.y - delta.position.w)
            } else if code_out.contains(OutCode::BOTTOM) {
                // Point is bellow the clipping region.
                (-e0.position.w - e0.position.y) / (delta.position.y + delta.position.w)
            } else if code_out.contains(OutCode::BACK) {
                // Point is behind the clipping region.
                (e0.position.w - e0.position.z) / (delta.position.z - delta.position.w)
            } else {
                // Point is ahead the clipping region.
                (-e0.position.w - e0.position.z) / (delta.position.z + delta.position.w)
            };
            let mut intersection = (1.0 - alpha) * e0 + alpha * e1;

            // Quick fix for floation point accuracy problems.
            if code_out.contains(OutCode::RIGHT) {
                intersection.position.x = intersection.position.w;
            } else if code_out.contains(OutCode::LEFT) {
                intersection.position.x = -intersection.position.w;
            } else if code_out.contains(OutCode::TOP) {
                intersection.position.y = intersection.position.w;
            } else if code_out.contains(OutCode::BOTTOM) {
                intersection.position.y = -intersection.position.w;
            } else if code_out.contains(OutCode::BACK) {
                intersection.position.z = intersection.position.w;
            } else {
                intersection.position.z = -intersection.position.w;
            };

            // Move the selected outside point to the intersection.
            if code_out == code0 {
                e0 = intersection;
                code0 = compute_outcode(e0.position);
            } else if code_out == code1 {
                e1 = intersection;
                code1 = compute_outcode(e1.position);
            }
        }
    }
}

/// Clips the triangle to the front-face or culls it to the back-face.
pub fn clip_triangle(triangle: Triangle<ColorVertex>) -> ClippedTriangle<ColorVertex> {
    // Cull tests
    if  triangle.0.position.x < -triangle.0.position.w
        && triangle.1.position.x < -triangle.1.position.w
        && triangle.2.position.x < -triangle.2.position.w
    {
        return ClippedTriangle::Empty;
    }
    if triangle.0.position.x > triangle.0.position.w
        && triangle.1.position.x > triangle.1.position.w
        && triangle.2.position.x > triangle.2.position.w
    {
        return ClippedTriangle::Empty;
    }
    if triangle.0.position.y < -triangle.0.position.w
        && triangle.1.position.y < -triangle.1.position.w
        && triangle.2.position.y < -triangle.2.position.w
    {
        return ClippedTriangle::Empty;
    }
    if triangle.0.position.y > triangle.0.position.w
        && triangle.1.position.y > triangle.1.position.w
        && triangle.2.position.y > triangle.2.position.w
    {
        return ClippedTriangle::Empty;
    }
    if triangle.0.position.z < -triangle.0.position.w
        && triangle.1.position.z < -triangle.1.position.w
        && triangle.2.position.z < -triangle.2.position.w
    {
        return ClippedTriangle::Empty;
    }
    if triangle.0.position.z > triangle.0.position.w
        && triangle.1.position.z > triangle.1.position.w
        && triangle.2.position.z > triangle.2.position.w
    {
        return ClippedTriangle::Empty;
    }

    if triangle.0.position.z < -triangle.0.position.w {
        if triangle.1.position.z < -triangle.1.position.w {
            clip_two_vertices(triangle.0, triangle.1, triangle.2)
        }
        else if triangle.2.position.z < -triangle.2.position.w {
            clip_two_vertices(triangle.0, triangle.2, triangle.1)
        }
        else {
            clip_one_vertex(triangle.0, triangle.1, triangle.2)
        }
    }
    else if triangle.1.position.z < -triangle.1.position.w {
        if triangle.2.position.z < -triangle.2.position.w {
            clip_two_vertices(triangle.1, triangle.2, triangle.0)
        }
        else {
            clip_one_vertex(triangle.1, triangle.0, triangle.2)
        }
    }
    else {
        ClippedTriangle::One(triangle)
    }
}

/// Clips a single vertex from the triangle. Creates two new triangles.
fn clip_one_vertex(v0: ColorVertex, v1: ColorVertex, v2: ColorVertex) -> ClippedTriangle<ColorVertex> {
    let delta1 = v1.position - v0.position;
    let delta2 = v2.position - v0.position;

    let alpha1 = -(v0.position.w + v0.position.z) / (delta1.z + delta1.w);
    let alpha2 = -(v0.position.w + v0.position.z) / (delta2.z + delta2.w);

    let v0a = ((1.0 - alpha1) * v0) + (alpha1 * v1);
    let v0b = ((1.0 - alpha2) * v0) + (alpha2 * v2);

    ClippedTriangle::Two(Triangle(v0a, v1, v2), Triangle(v0b, v0a, v2))
}

/// Clips two vertices from the triangle. Creates only one triangle.
fn clip_two_vertices(v0: ColorVertex, v1: ColorVertex, v2: ColorVertex) -> ClippedTriangle<ColorVertex> {
    let delta1 = v2.position - v0.position;
    let delta2 = v2.position - v1.position;

    let alpha1 = -(v0.position.w + v0.position.z) / (delta1.z + delta1.w);
    let alpha2 = -(v1.position.w + v1.position.z) / (delta2.z + delta2.w);

    let v0 = ((1.0 - alpha1) * v0) + (alpha1 * v2);
    let v1 = ((1.0 - alpha2) * v1) + (alpha2 * v2);

    ClippedTriangle::One(Triangle(v0, v1, v2))
}