use bitflags::bitflags;
use cgmath::Vector4;

// TODO: Use clip space coordinates instead of NDC.

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

fn compute_outcode(position: Vector4<f32>) -> OutCode {
    let mut code: OutCode = OutCode::INSIDE;

    if position.x < -position.w {
        code |= OutCode::LEFT;
    }
    else if position.x > position.w {
        code |= OutCode::RIGHT;
    }
    if position.y < -position.w {
        code |= OutCode::BOTTOM;
    }
    else if position.y > position.w {
        code |= OutCode::TOP;
    }
    if position.z < -position.w {
        code |= OutCode::FRONT;
    }
    else if position.z > position.w {
        code |= OutCode::BACK;
    }

    code
}

pub fn clip_line(line: (Vector4<f32>, Vector4<f32>)) -> Option<(Vector4<f32>,Vector4<f32>)> {
    let (mut e0, mut e1) = line;

    // Calculate where the endpoints are in relation to the clipping rectangle.
    let mut code0 = compute_outcode(line.0);
    let mut code1 = compute_outcode(line.1);
    
    loop {
        if (code0 | code1) == OutCode::INSIDE {
            // Both endpoints are inside the clip region. Trivial accept.
            break Some((e0, e1));
        }
        else if (code0 & code1) != OutCode::INSIDE {
            // Both endpoints share an outside region. In other words,
            // the line is outside. Trivial reject.
            break None;
        }
        else {
            // Calculate the segment from an outside point to an intersection to clip.

            // Pick one of the outside segments.
            let code_out = OutCode::max(code0, code1);

            // Find intersection point.
            let delta = e1 - e0;
            let alpha = if code_out.contains(OutCode::RIGHT) {
                // Point is right to the clipping region.
                (e0.w - e0.x) / (delta.x - delta.w)
            }
            else if code_out.contains(OutCode::LEFT) {
                // Point is left to the clipping region.
                (-e0.w - e0.x) / (delta.x + delta.w)
            }
            else if code_out.contains(OutCode::TOP) {
                // Point is above the clipping region.
                (e0.w - e0.y) / (delta.y - delta.w)
            }
            else if code_out.contains(OutCode::BOTTOM) {
                // Point is bellow the clipping region.
                (-e0.w - e0.y) / (delta.y + delta.w)
            }
            else if code_out.contains(OutCode::BACK) {
                // Point is behind the clipping region.
                (e0.w - e0.z) / (delta.z - delta.w)
            }
            else {
                // Point is ahead the clipping region.
                (-e0.w - e0.z) / (delta.z + delta.w)
            };
            let mut intersection = (1.0 - alpha) * e0 + alpha * e1;

            // Quick fix for floation point accuracy problems.
            if code_out.contains(OutCode::RIGHT) {
                intersection.x = intersection.w;
            }
            else if code_out.contains(OutCode::LEFT) {
                intersection.x = -intersection.w;
            }
            else if code_out.contains(OutCode::TOP) {
                intersection.y = intersection.w;
            }
            else if code_out.contains(OutCode::BOTTOM) {
                intersection.y = -intersection.w;
            }
            else if code_out.contains(OutCode::BACK) {
                intersection.z = intersection.w;
            }
            else {
                intersection.z = -intersection.w;
            };

            // Move the selected outside point to the intersection.
            if code_out == code0 {
                e0 = intersection;
                code0 = compute_outcode(e0);
            }
            else if code_out == code1 {
                e1 = intersection;
                code1 = compute_outcode(e1);
            }
        }
    }
}
