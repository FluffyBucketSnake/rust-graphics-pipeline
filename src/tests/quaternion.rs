use crate::math::Quaternion;

#[test]
fn basic_arith() {
    let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(q + Quaternion::k(), Quaternion::new(1.0, 2.0, 4.0, 4.0));
    assert_eq!(q - Quaternion::k(), Quaternion::new(1.0, 2.0, 2.0, 4.0));

    assert_eq!(q + Quaternion::j(), Quaternion::new(1.0, 3.0, 3.0, 4.0));
    assert_eq!(q - Quaternion::j(), Quaternion::new(1.0, 1.0, 3.0, 4.0));

    assert_eq!(q + Quaternion::i(), Quaternion::new(2.0, 2.0, 3.0, 4.0));
    assert_eq!(q - Quaternion::i(), Quaternion::new(0.0, 2.0, 3.0, 4.0));
    
    assert_eq!(q + 1.0, Quaternion::new(1.0, 2.0, 3.0, 5.0));
    assert_eq!(q - 1.0, Quaternion::new(1.0, 2.0, 3.0, 3.0));
    assert_eq!(q / 2.0, Quaternion::new(0.5, 1.0, 1.5, 2.0));
}

#[test]
fn multiplication() {
    let i = Quaternion::i();
    let j = Quaternion::j();
    let k = Quaternion::k();
    let one = Quaternion::identity();

    assert_eq!(one * one, one);
    assert_eq!(one * i, i);
    assert_eq!(one * j, j);
    assert_eq!(one * k, k);
    assert_eq!(i * one, i);
    assert_eq!(i * i, -one);
    assert_eq!(i * j,  k);
    assert_eq!(i * k, -j);
    assert_eq!(j * one, j);
    assert_eq!(j * i, -k);
    assert_eq!(j * j, -one);
    assert_eq!(j * k,  i);
    assert_eq!(k * one, k);
    assert_eq!(k * i,  j);
    assert_eq!(k * j, -i);
    assert_eq!(k * k, -one);
}
