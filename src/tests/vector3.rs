use crate::math::{Matrix,Vec3f};

#[test]
fn dot() {
    let v1 = Vec3f::new(1.0, 2.0, 3.0);
    let v2 = Vec3f::new(4.0, 5.0, 6.0);
    let v3 = Vec3f::new(7.0, 8.0, 9.0);

    // Vector on vector.
    assert_eq!(v1.dot(&v2), (1.0 * 4.0) + (2.0 * 5.0) + (3.0 * 6.0));
    assert_eq!(v2.dot(&v3), (4.0 * 7.0) + (5.0 * 8.0) + (6.0 * 9.0));
    assert_eq!(v1.dot(&v3), (1.0 * 7.0) + (2.0 * 8.0) + (3.0 * 9.0));

    // Vector on zero vector.
    assert_eq!(v1.dot(&Vec3f::zero()), 0.0);

    // Vector on one vector.
    assert_eq!(v1.dot(&Vec3f::one()), 6.0);

    // Commutative property.
    assert_eq!(v1.dot(&v2), v2.dot(&v1));

    // Associative property.
    assert_eq!(v1.dot(&(v2 + v3)), v1.dot(&v2) + v1.dot(&v3));

    // Scalar multiplicative.
    assert_eq!((2.0 * v1).dot(&(3.0 * v2)), (2.0 * 3.0) * v1.dot(&v2));

    // Geometrical properties.
    assert_eq!(Vec3f::positive_x().dot(&Vec3f::positive_x()), 1.0);
    assert_eq!(Vec3f::negative_y().dot(&Vec3f::positive_y()), -1.0);
    assert_eq!(Vec3f::positive_z().dot(&Vec3f::positive_y()), 0.0);
}

#[test]
fn cross() {
    let i = Vec3f::positive_x();
    let j = Vec3f::positive_y();
    let k = Vec3f::positive_z();

    // Vector on vector.
    assert_eq!(i.cross(&j), k);
    assert_eq!(i.cross(&(-j)), -k);
    assert_eq!(i.cross(&i), Vec3f::zero());
    assert_eq!(i.cross(&(-i)), Vec3f::zero());

    // Scalar multiplication.
    assert_eq!((2.0 * i).cross(&(3.0 * j)), 6.0 * k);

    // Anticommutative property.
    assert_eq!(i.cross(&j), -j.cross(&i));
}

#[test]
fn transformations() {
    let v = Vec3f::one();

    // 
    // Scaling
    //

    let uscale = Matrix::scale_uniform(2.0);
    assert_eq!(uscale * v, 2.0 * v);

    let scale = Matrix::scale(1.0, 2.0, 3.0);
    assert_eq!(scale * v, Vec3f::new(1.0, 2.0, 3.0));

    //
    // Rotation
    //
    let theta = std::f32::consts::FRAC_PI_2;

    // Rotation around the x-axis.
    let rotation_x = Matrix::rotate_x(theta);

    assert_eq!(rotation_x * Vec3f::positive_x(), Vec3f::positive_x());
    assert_eq!(rotation_x * Vec3f::negative_x(), Vec3f::negative_x());
    // Float precision problems...
    // TODO: Make a custom macro for float/vector comparisons.
    assert_eq!(rotation_x * Vec3f::positive_y(), Vec3f::new(0.0,  theta.cos(),  theta.sin()));
    assert_eq!(rotation_x * Vec3f::negative_y(), Vec3f::new(0.0, -theta.cos(), -theta.sin()));
    assert_eq!(rotation_x * Vec3f::positive_z(), Vec3f::new(0.0, -theta.sin(),  theta.cos()));
    assert_eq!(rotation_x * Vec3f::negative_z(), Vec3f::new(0.0,  theta.sin(), -theta.cos()));

    // Rotation around the y-axis.
    let rotation_y = Matrix::rotate_y(std::f32::consts::FRAC_PI_2);

    assert_eq!(rotation_y * Vec3f::positive_x(), Vec3f::new( theta.cos(), 0.0, -theta.sin()));
    assert_eq!(rotation_y * Vec3f::negative_x(), Vec3f::new(-theta.cos(), 0.0,  theta.sin()));
    assert_eq!(rotation_y * Vec3f::positive_y(), Vec3f::positive_y());
    assert_eq!(rotation_y * Vec3f::negative_y(), Vec3f::negative_y());
    assert_eq!(rotation_y * Vec3f::positive_z(), Vec3f::new( theta.sin(), 0.0,  theta.cos()));
    assert_eq!(rotation_y * Vec3f::negative_z(), Vec3f::new(-theta.sin(), 0.0, -theta.cos()));

    // Rotation around the z-axis.
    let rotation_z = Matrix::rotate_z(std::f32::consts::FRAC_PI_2);

    assert_eq!(rotation_z * Vec3f::positive_x(), Vec3f::new( theta.cos(),  theta.sin(), 0.0));
    assert_eq!(rotation_z * Vec3f::negative_x(), Vec3f::new(-theta.cos(), -theta.sin(), 0.0));
    assert_eq!(rotation_z * Vec3f::positive_y(), Vec3f::new(-theta.sin(),  theta.cos(), 0.0));
    assert_eq!(rotation_z * Vec3f::negative_y(), Vec3f::new( theta.sin(), -theta.cos(), 0.0));
    assert_eq!(rotation_z * Vec3f::positive_z(), Vec3f::positive_z());
    assert_eq!(rotation_z * Vec3f::negative_z(), Vec3f::negative_z());
}
