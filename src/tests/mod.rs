use crate::math::Vec2f;

mod vector2 {
    use super::*;

    #[test]
    fn dot() {
        let v1 = Vec2f::new(1.0, 2.0);
        let v2 = Vec2f::new(3.0, 4.0);
        let v3 = Vec2f::new(5.0, 6.0);

        // Vector on vector.
        assert_eq!(v1.dot(&v2), (1.0 * 3.0) + (2.0 * 4.0));
        assert_eq!(v2.dot(&v3), (3.0 * 5.0) + (4.0 * 6.0));
        assert_eq!(v1.dot(&v3), (1.0 * 5.0) + (2.0 * 6.0));

        // Vector on zero vector.
        assert_eq!(v1.dot(&Vec2f::zero()), 0.0);

        // Vector on one vector.
        assert_eq!(v1.dot(&Vec2f::one()), 3.0);

        // Commutative property.
        assert_eq!(v1.dot(&v2), v2.dot(&v1));

        // Associative property.
        assert_eq!(v1.dot(&(v2 + v3)), v1.dot(&v2) + v1.dot(&v3));

        // Scalar multiplicative.
        assert_eq!((2.0 * v1).dot(&(3.0 * v2)), (2.0 * 3.0) * v1.dot(&v2));

        // Geometrical properties.
        assert_eq!(Vec2f::positive_x().dot(&Vec2f::positive_x()), 1.0);
        assert_eq!(Vec2f::negative_x().dot(&Vec2f::positive_x()), -1.0);
        assert_eq!(Vec2f::positive_x().dot(&Vec2f::positive_y()), 0.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec2f::positive_x();
        let v2 = Vec2f::positive_y();

        // Vector on vector.
        assert_eq!(v1.cross(&v2), 1.0);
        assert_eq!(v1.cross(&(-v2)), -1.0);
        assert_eq!(v1.cross(&v1), 0.0);
        assert_eq!(v1.cross(&(-v1)), 0.0);

        // Scalar multiplication.
        assert_eq!((2.0 * v1).cross(&(3.0 * v2)), 6.0);

        // Anticommutative property.
        assert_eq!(v1.cross(&v2), -v2.cross(&v1));
    }
}
