use pretty_assertions::assert_eq as assert_eq_pr;

use bc_utils::other::*;

#[test]
fn roll_slice_res_1() {
    let v = &mut [1, 2, 3];
    roll_slice1(v, 1);
    assert_eq_pr!(v, &[3, 1, 2]);
}

#[test]
fn roll_slice_res_2() {
    let v = &mut [1, 2, 3];
    roll_slice1(v, -1);
    assert_eq_pr!(v, &[2, 3, 1]);
}

#[test]
fn roll_slice_link_1() {
    let v = &mut [&1, &2, &3];
    roll_slice1(v, 1);
    assert_eq_pr!(v, &[&3, &1, &2]);
}

#[test]
fn roll_slice_link_2() {
    let v = &mut [&1, &2, &3];
    roll_slice1(v, -1);   
    assert_eq_pr!(v, &[&2, &3, &1]);
}

#[test]
fn g_roll_slice_res_1() {
    assert_eq_pr!(g_roll_slice1(&mut [1, 2, 3], 1), &[3, 1, 2]);
}

#[test]
fn g_roll_slice_res_2() {
    assert_eq_pr!(g_roll_slice1(&mut [1, 2, 3], -1), &[2, 3, 1]);
}

#[test]
fn g_roll_slice_link_1() {
    assert_eq_pr!(g_roll_slice1(&mut [&1, &2, &3], 1), &[&3, &1, &2]);
}

#[test]
fn g_roll_slice_link_2() {
    assert_eq_pr!(g_roll_slice1(&mut [&1, &2, &3], -1), &[&2, &3, &1]);
}

#[test]
fn coll1_roll_replace_el_res_neg_1() {
    assert_eq_pr!(
        coll1_roll_replace_el::<Vec<i8>, _, _,>(
            &mut [1, 2, 3],
            -1, 
            4,
        ), 
        vec![2, 3, 4],
    );
}

#[test]
fn coll1_roll_replace_el_res_pos_2() {
    assert_eq_pr!(
        coll1_roll_replace_el::<Vec<i8>, _, _,>(
            &mut [1, 2, 3],
            1, 
            4,
        ), 
        vec![4, 1, 2],
    );
}

#[test]
fn coll1_roll_replace_el_link_1() {
    assert_eq_pr!(
        coll1_roll_replace_el::<Vec<&i8>, i8, &i8>(
            &mut [&1, &2, &3],
            -1, 
            &4,
        ),
        vec![&2, &3, &4],
    );
}

#[test]
fn coll1_roll_replace_el_link_2() {
    assert_eq_pr!(
        coll1_roll_replace_el::<Vec<&i8>, i8, _,>(
            &mut [&1, &2, &3],
            1, 
            &4,
        ),
        vec![&4, &1, &2],
    );
}

#[test]
fn lstrip_res_1() {
    assert_eq_pr!(lstrip("hello world", 'w'), "world");
}

#[test]
fn rstrip_res_1() {
    assert_eq_pr!(rstrip("hello world", 'w'), "hello w");
}

#[test]
fn transpose_res_1() {
    assert_eq_pr!(transpose(vec![vec![1, 2, 3], vec![1, 2, 3],]), vec![vec![1, 1,], vec![2, 2,], vec![3, 3,]])
}