use crate::math::{Matrix,Vec4f};

#[test]
fn scalar_algebra() {
    let v = Vec4f::new(1.0, 2.0, 3.0, 4.0);
    let s = 5.0;

    assert_eq!(v * s, Vec4f::new(1.0 * 5.0, 2.0 * 5.0, 3.0 * 5.0, 4.0 * 5.0));
    assert_eq!(v / s, Vec4f::new(1.0 / 5.0, 2.0 / 5.0, 3.0 / 5.0, 4.0 / 5.0));
}

#[test]
fn vector_algebra() {
    let v1 = Vec4f::new(1.0, 2.0, 3.0, 4.0);
    let v2 = Vec4f::new(5.0, 6.0, 7.0, 8.0);

    assert_eq!(v1 + v2, Vec4f::new(1.0 + 5.0, 2.0 + 6.0, 3.0 + 7.0, 4.0 + 8.0));
    assert_eq!(v1 - v2, Vec4f::new(1.0 - 5.0, 2.0 - 6.0, 3.0 - 7.0, 4.0 - 8.0));
    assert_eq!(v1 * v2, Vec4f::new(1.0 * 5.0, 2.0 * 6.0, 3.0 * 7.0, 4.0 * 8.0));
    assert_eq!(v1 / v2, Vec4f::new(1.0 / 5.0, 2.0 / 6.0, 3.0 / 7.0, 4.0 / 8.0));
}

#[test]
fn dot() {
    let v1 = Vec4f::new(1.0,  2.0,  3.0,  4.0);
    let v2 = Vec4f::new(5.0,  6.0,  7.0,  8.0);
    let v3 = Vec4f::new(9.0, 10.0, 11.0, 12.0);

    // Vector on vector.
    assert_eq!(v1.dot(&v2), (1.0 * 5.0) + (2.0 *  6.0) +  (3.0 * 7.0) + (4.0 *  8.0));
    assert_eq!(v2.dot(&v3), (5.0 * 9.0) + (6.0 * 10.0) + (7.0 * 11.0) + (8.0 * 12.0));
    assert_eq!(v1.dot(&v3), (1.0 * 9.0) + (2.0 * 10.0) + (3.0 * 11.0) + (4.0 * 12.0));

    // Vector on zero vector.
    assert_eq!(v1.dot(&Vec4f::zero()), 0.0);

    // Vector on one vector.
    assert_eq!(v1.dot(&Vec4f::one()), 10.0);

    // Commutative property.
    assert_eq!(v1.dot(&v2), v2.dot(&v1));

    // Associative property.
    assert_eq!(v1.dot(&(v2 + v3)), v1.dot(&v2) + v1.dot(&v3));

    // Scalar multiplicative.
    assert_eq!((2.0 * v1).dot(&(3.0 * v2)), (2.0 * 3.0) * v1.dot(&v2));

    // Geometrical properties.
    assert_eq!( Vec4f::unitx().dot(&Vec4f::unitx()),  1.0);
    assert_eq!(-Vec4f::unity().dot(&Vec4f::unity()), -1.0);
    assert_eq!( Vec4f::unitz().dot(&Vec4f::unity()),  0.0);
}

#[test]
fn transformations() {
    let v = Vec4f::one();

    // 
    // Scaling
    //

    let uscale = Matrix::scale_uniform(2.0);
    assert_eq!(uscale * v, Vec4f::new(2.0, 2.0, 2.0, 1.0));

    let scale = Matrix::scale(1.0, 2.0, 3.0);
    assert_eq!(scale * v, Vec4f::new(1.0, 2.0, 3.0, 1.0));

    //
    // Rotation
    //
    let theta = std::f32::consts::FRAC_PI_2;

    // Rotation around the x-axis.
    let rotation_x = Matrix::rotate_x(theta);

    // Float precision problems...
    // TODO: Make a custom macro for float/vector comparisons.
    assert_eq!(rotation_x *  Vec4f::unitx(),  Vec4f::unitx());
    assert_eq!(rotation_x * -Vec4f::unitx(), -Vec4f::unitx());
    assert_eq!(rotation_x *  Vec4f::unity(),  Vec4f::new(0.0,  theta.cos(),  theta.sin(), 0.0));
    assert_eq!(rotation_x * -Vec4f::unity(),  Vec4f::new(0.0, -theta.cos(), -theta.sin(), 0.0));
    assert_eq!(rotation_x *  Vec4f::unitz(),  Vec4f::new(0.0, -theta.sin(),  theta.cos(), 0.0));
    assert_eq!(rotation_x * -Vec4f::unitz(),  Vec4f::new(0.0,  theta.sin(), -theta.cos(), 0.0));

    // Rotation around the y-axis.
    let rotation_y = Matrix::rotate_y(std::f32::consts::FRAC_PI_2);

    assert_eq!(rotation_y *  Vec4f::unitx(),  Vec4f::new( theta.cos(), 0.0, -theta.sin(), 0.0));
    assert_eq!(rotation_y * -Vec4f::unitx(),  Vec4f::new(-theta.cos(), 0.0,  theta.sin(), 0.0));
    assert_eq!(rotation_y *  Vec4f::unity(),  Vec4f::unity());
    assert_eq!(rotation_y * -Vec4f::unity(), -Vec4f::unity());
    assert_eq!(rotation_y *  Vec4f::unitz(),  Vec4f::new( theta.sin(), 0.0,  theta.cos(), 0.0));
    assert_eq!(rotation_y * -Vec4f::unitz(),  Vec4f::new(-theta.sin(), 0.0, -theta.cos(), 0.0));

    // Rotation around the z-axis.
    let rotation_z = Matrix::rotate_z(std::f32::consts::FRAC_PI_2);

    assert_eq!(rotation_z *  Vec4f::unitx(),  Vec4f::new( theta.cos(),  theta.sin(), 0.0, 0.0));
    assert_eq!(rotation_z * -Vec4f::unitx(),  Vec4f::new(-theta.cos(), -theta.sin(), 0.0, 0.0));
    assert_eq!(rotation_z *  Vec4f::unity(),  Vec4f::new(-theta.sin(),  theta.cos(), 0.0, 0.0));
    assert_eq!(rotation_z * -Vec4f::unity(),  Vec4f::new( theta.sin(), -theta.cos(), 0.0, 0.0));
    assert_eq!(rotation_z *  Vec4f::unitz(),  Vec4f::unitz());
    assert_eq!(rotation_z * -Vec4f::unitz(), -Vec4f::unitz());
}
