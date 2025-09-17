use core::f64;

use bc_utils::nums::*;

#[test]
fn avg_res_1() {
    assert_eq!(
        avg(&[1.0, 2.0, 3.0]),
        2.0,
    )
}

#[test]
fn avg_link_1() {
    assert_eq!(
        avg::<f64, &f64>(&[&1.0, &2.0, &3.0]),
        2.0
    )
}

#[test]
fn avg_with_res_1() {
    assert_eq!(
        avg_with(&1.0, &[2.0, 3.0]),
        2.0,
    )
}

#[test]
fn avg_with_link_1() {
    assert_eq!(
        avg_with::<f64>(&1.0, &[2.0, 3.0]),
        2.0
    )
}

#[test]
fn nz_res_1() {
    assert_eq!(
        nz(f64::NAN, 0.0),
        0.0,
    )
}

#[test]
fn nz_link_1() {
    assert_eq!(
        nz::<f64, &f64>(&f64::NAN, &0.0),
        &0.0,
    )
}

#[test]
fn coll_nz_res_1() {
    assert_eq!(
        coll_nz::<Vec<f64>, _, _,>(&[1.0, f64::NAN,], 2.0),
        vec![1.0, 2.0],
    )
}

#[test]
fn coll_nz_link_1() {
    assert_eq!(
        coll_nz::<Vec<&f64>, f64, &f64,>(&[&1.0, &f64::NAN,], &2.0),
        vec![&1.0, &2.0],
    )
}

#[test]
fn normalize_res_1() {
    assert_eq!(normalize(&[-10.0, -20.0,], -10.0, &0.0, &1.0,), 1.0,)
}

#[test]
fn normalize_link_1() {
    assert_eq!(normalize(&[&-10.0, &-20.0,], &-10.0, &0.0, &1.0,), 1.0,)
}

#[test]
fn dz_res_1() {
    assert_eq!(dz(0.0), 1e-10,)
}

#[test]
fn coll_dropnan_res_1() {
    assert_eq!(
        coll_drop_nan::<f64, _, Vec<f64>>(&[1.0, 2.0, f64::NAN]),
        vec![1.0, 2.0]
    );
}

#[test]
fn coll_dropnan_link_1() {
    assert_eq!(
        coll_drop_nan::<f64, _, Vec<&f64>>(&[&1.0, &2.0, &f64::NAN]),
        vec![&1.0, &2.0]
    );
}

#[test]
fn abs_res_1() {
    assert_eq!(abs(-1), 1)
}

#[test]
fn abs_res_2() {
    assert_eq!(abs(-1.0), 1.0)
}

#[test]
fn abs_link_1() {
    assert_eq!(abs::<i8, _>(&-1), 1)
}

#[test]
fn abs_link_2() {
    assert_eq!(abs::<f64, _>(&-1.0), 1.0)
}

#[test]
fn round_f_res_1() {
    assert_eq!(1.123, round_f(1.123456, &3),)
}

#[test]
#[allow(clippy::needless_borrows_for_generic_args)]
fn round_f_link_1() {
    assert_eq!(1.123, round_f(&1.123456, &3),)
}

#[test]
fn coll_comp_res_1() {
    assert_eq!(
        coll_comp::<Vec<&f64>, _, _,>(&[1.0, -1.0, 0.0], |v| *v > 0.0),
        vec![&1.0,]
    )
}

#[test]
fn coll_comp_res_2() {
    assert_eq!(
        coll_comp::<Vec<&f64>, _, _,>(&[1.0, -1.0, 0.0], |v| *v < 0.0),
        vec![&-1.0,]
    )
}

#[test]
fn coll_comp_res_3() {
    assert_eq!(
        coll_comp::<Vec<&f64>, _, _,>(&[1.0, -1.0, 0.0], |v| *v == 0.0),
        vec![&0.0,]
    )
}

#[test]
fn coll_comp_link_1() {
    assert_eq!(
        coll_comp::<Vec<&f64>, _, _,>(&[&1.0, &-1.0, &0.0], |v| *v > 0.0),
        vec![&1.0,]
    )
}

#[test]
fn coll_comp_link_2() {
    assert_eq!(
        coll_comp::<Vec<&f64>, _, _,>(&[&1.0, &-1.0, &0.0], |v| *v < 0.0),
        vec![&-1.0,]
    )
}

#[test]
fn coll_comp_link_3() {
    assert_eq!(
        coll_comp::<Vec<&f64>, _, _,>(&[&1.0, &-1.0, &0.0], |v| *v == 0.0),
        vec![&0.0,]
    )
}

#[test]
fn sign_res_1() {
    assert_eq!(1.0, sign(5.0),);
}

#[test]
fn sign_res_2() {
    assert_eq!(-1.0, sign(-5.0),);
}

#[test]
fn sign_res_3() {
    assert_eq!(0.0, sign(0.0),);
}
