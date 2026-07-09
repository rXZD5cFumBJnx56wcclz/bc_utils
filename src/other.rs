use std::borrow::Borrow;

use num_traits::ToPrimitive;

pub fn lstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s.chars().enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[cut_index..]
}

pub fn rstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s.chars().rev().enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[..s.len() - cut_index]
}

pub fn roll_slice1<T>(
    v: &mut [T],
    shift: i32,
) {
    let shift_usize = shift.abs().to_usize().unwrap();

    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
}

pub fn g_roll_slice1<'a, T>(
    v: &'a mut [T],
    shift: i32,
) -> &'a [T] {
    let shift_usize = shift.abs().to_usize().unwrap();

    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
    v
}

pub fn coll1_roll_replace_el<'a, C, T, V>(
    slice: &mut [V],
    shift: i32,
    to_replace: V,
) -> C
where
    T: 'a,
    V: Borrow<T>,
    V: Copy,
    C: FromIterator<V>,
{
    let len = slice.len();
    roll_slice1(slice, shift);
    let iter_ = slice.iter();
    let shift_usize = shift.abs().to_usize().unwrap();

    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => {
            let num_need = shift_usize - 1;
            iter_
                .enumerate()
                .map(|(i, v)| {
                    if i <= num_need {
                        to_replace
                    } else {
                        *v
                    }
                })
                .collect()
        }
        std::cmp::Ordering::Less => {
            let num_need = (len as i32 + shift) as usize;
            iter_
                .enumerate()
                .map(|(i, v)| {
                    if i >= num_need {
                        to_replace
                    } else {
                        *v
                    }
                })
                .collect()
        }
        std::cmp::Ordering::Equal => iter_.copied().collect(),
    }
}

pub fn transpose<T>(mut value: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..value.first().unwrap_or(&vec![]).len())
        .map(|_| {
            (&mut value)
                .into_iter()
                .map(|v| v.remove(0))
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

pub fn transpose_set<T>(value: &mut Vec<Vec<T>>) {
    let mut vec = vec![];
    for _ in 0..value.first().unwrap_or(&vec![]).len() {
        vec.push(value.iter_mut().map(|v| v.remove(0)).collect::<Vec<T>>());
    }
    *value = vec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as assert_eq_pr;

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
            coll1_roll_replace_el::<Vec<i8>, _, _>(&mut [1, 2, 3], -1, 4,),
            vec![2, 3, 4],
        );
    }

    #[test]
    fn coll1_roll_replace_el_res_pos_2() {
        assert_eq_pr!(
            coll1_roll_replace_el::<Vec<i8>, _, _>(&mut [1, 2, 3], 1, 4,),
            vec![4, 1, 2],
        );
    }

    #[test]
    fn coll1_roll_replace_el_link_1() {
        assert_eq_pr!(
            coll1_roll_replace_el::<Vec<&i8>, i8, &i8>(&mut [&1, &2, &3], -1, &4,),
            vec![&2, &3, &4],
        );
    }

    #[test]
    fn coll1_roll_replace_el_link_2() {
        assert_eq_pr!(
            coll1_roll_replace_el::<Vec<&i8>, i8, _>(&mut [&1, &2, &3], 1, &4,),
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
        assert_eq_pr!(
            transpose(vec![vec![1, 2, 3], vec![1, 2, 3],]),
            vec![vec![1, 1,], vec![2, 2,], vec![3, 3,]]
        )
    }
    #[test]
    fn transpose_set_res_1() {
        let mut vec = vec![vec![1, 2, 3], vec![1, 2, 3]];
        transpose_set(&mut vec);
        assert_eq_pr!(vec, vec![vec![1, 1,], vec![2, 2,], vec![3, 3,]])
    }
}
