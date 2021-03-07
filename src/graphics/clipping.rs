use crate::math::Vec3f;

type OutCode = u8;

const INSIDE: OutCode = 0;
const LEFT: OutCode = 1;
const RIGHT: OutCode = 2;
const BOTTOM: OutCode = 4;
const TOP: OutCode = 8;
const FRONT: OutCode = 16;
const BACK: OutCode = 32;

const X_MIN: f32 = -1.0;
const Y_MIN: f32 = -1.0;
const Z_MIN: f32 = -1.0;
const X_MAX: f32 = 1.0;
const Y_MAX: f32 = 1.0;
const Z_MAX: f32 = 1.0;

fn compute_outcode(position: Vec3f) -> OutCode {
    let mut code: OutCode = INSIDE;

    if position.x < X_MIN {
        code |= LEFT;
    }
    else if position.x > X_MAX {
        code |= RIGHT;
    }
    if position.y < Y_MIN {
        code |= BOTTOM;
    }
    else if position.y > Y_MAX {
        code |= TOP;
    }
    if position.z < Z_MIN {
        code |= BACK;
    }
    else if position.z > Z_MAX {
        code |= FRONT;
    }

    code
}

pub fn clip_line(line: (Vec3f, Vec3f)) -> Option<(Vec3f,Vec3f)> {
    let (mut e0, mut e1) = line;

    // Calculate where the endpoints are in relation to the clipping rectangle.
    let mut code0 = compute_outcode(line.0);
    let mut code1 = compute_outcode(line.1);
    
    loop {
        if (code0 | code1) == INSIDE {
            // Both endpoints are inside the clip region. Trivial accept.
            break Some((e0, e1));
        }
        else if (code0 & code1) != INSIDE {
            // Both endpoints share an outside region. In other words,
            // the line is outside. Trivial reject.
            break None;
        }
        else {
            // Calculate the segment from an outside point to an intersection to clip.

            // Pick one of the outside segments.
            let code_out = OutCode::max(code0, code1);

            // Find intersection point.
            let intersection = if (code_out & TOP) != INSIDE {
                // Point is above the clipping region.
                Vec3f {
                    x: e0.x + (e1.x - e0.x) * (Y_MAX - e0.y) / (e1.y - e0.y),
                    y: Y_MAX,
                    z: e0.z + (e1.z - e0.z) * (Y_MAX - e0.y) / (e1.y - e0.y),
                }
            }
            else if (code_out & BOTTOM) != INSIDE {
                // Point is bellow the clipping region.
                Vec3f {
                    x: e0.x + (e1.x - e0.x) * (Y_MIN - e0.y) / (e1.y - e0.y),
                    y: Y_MIN,
                    z: e0.z + (e1.z - e0.z) * (Y_MIN - e0.y) / (e1.y - e0.y),
                }
            }
            else if (code_out & RIGHT) != INSIDE {
                // Point is right to the clipping region.
                Vec3f {
                    x: X_MAX,
                    y: e0.y + (e1.y - e0.y) * (X_MAX - e0.x) / (e1.x - e0.x),
                    z: e0.z + (e1.z - e0.z) * (X_MAX - e0.x) / (e1.x - e0.x),
                }
            }
            else if (code_out & LEFT) != INSIDE {
                // Point is left to the clipping region.
                Vec3f {
                    x: X_MIN,
                    y: e0.y + (e1.y - e0.y) * (X_MIN - e0.x) / (e1.x - e0.x),
                    z: e0.z + (e1.z - e0.z) * (X_MIN - e0.x) / (e1.x - e0.x),
                }
            }
            else if (code_out & FRONT) != INSIDE {
                // Point is in front of the clipping region.
                Vec3f {
                    x: e0.x + (e1.x - e0.x) * (Z_MAX - e0.z) / (e1.z - e0.z),
                    y: e0.y + (e1.y - e0.y) * (Z_MAX - e0.z) / (e1.z - e0.z),
                    z: Z_MAX,
                }
            }
            else {
                // Point is behind the clipping region.
                Vec3f {
                    x: e0.x + (e1.x - e0.x) * (Z_MIN - e0.z) / (e1.z - e0.z),
                    y: e0.y + (e1.y - e0.y) * (Z_MIN - e0.z) / (e1.z - e0.z),
                    z: Z_MIN,
                }
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
