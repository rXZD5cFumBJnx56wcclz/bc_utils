use bc_utils::other::*;

#[test]
fn roll_slice_res_1() {
    let v = &mut [1, 2, 3];
    roll_slice1(v, &1);
    assert_eq!(v, &[3, 1, 2]);
}

#[test]
fn roll_slice_res_2() {
    let v = &mut [1, 2, 3];
    roll_slice1(v, &-1);
    assert_eq!(v, &[2, 3, 1]);
}

#[test]
fn roll_slice_link_1() {
    let v = &mut [&1, &2, &3];
    roll_slice1(v, &1);
    assert_eq!(v, &[&3, &1, &2]);
}

#[test]
fn roll_slice_link_2() {
    let v = &mut [&1, &2, &3];
    roll_slice1(v, &-1);   
    assert_eq!(v, &[&2, &3, &1]);
}

#[test]
fn g_roll_slice_res_1() {
    assert_eq!(g_roll_slice1(&mut [1, 2, 3], &1), &[3, 1, 2]);
}

#[test]
fn g_roll_slice_res_2() {
    assert_eq!(g_roll_slice1(&mut [1, 2, 3], &-1), &[2, 3, 1]);
}

#[test]
fn g_roll_slice_link_1() {
    assert_eq!(g_roll_slice1(&mut [&1, &2, &3], &1), &[&3, &1, &2]);
}

#[test]
fn g_roll_slice_link_2() {
    assert_eq!(g_roll_slice1(&mut [&1, &2, &3], &-1), &[&2, &3, &1]);
}

#[test]
fn coll1_roll_replace_el_res_neg_1() {
    assert_eq!(
        coll1_roll_replace_el::<Vec<i8>, _, _,>(
            &mut [1, 2, 3],
            &-1, 
            4,
        ), 
        vec![2, 3, 4],
    );
}

#[test]
fn coll1_roll_replace_el_res_pos_2() {
    assert_eq!(
        coll1_roll_replace_el::<Vec<i8>, _, _,>(
            &mut [1, 2, 3],
            &1, 
            4,
        ), 
        vec![4, 1, 2],
    );
}

#[test]
fn coll1_roll_replace_el_link_1() {
    assert_eq!(
        coll1_roll_replace_el::<Vec<&i8>, i8, &i8>(
            &mut [&1, &2, &3],
            &-1, 
            &4,
        ),
        vec![&2, &3, &4],
    );
}

#[test]
fn coll1_roll_replace_el_link_2() {
    assert_eq!(
        coll1_roll_replace_el::<Vec<&i8>, i8, _,>(
            &mut [&1, &2, &3],
            &1, 
            &4,
        ),
        vec![&4, &1, &2],
    );
}

#[test]
fn lstrip_res_1() {
    assert_eq!(lstrip("hello world", 'w'), "world");
}

#[test]
fn rstrip_res_1() {
    assert_eq!(rstrip("hello world", 'w'), "hello w");
}