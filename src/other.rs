use std::{borrow::Borrow, collections::hash_map::Entry, hash::Hash};

use bc_utils_lg::types::maps::{MAP, MAP_LINK};
use num_traits::ToPrimitive;

pub fn lstrip(s: &'static str, cut_before: char) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s.chars().enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[cut_index..]
}

pub fn rstrip(s: &'static str, cut_before: char) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s.chars().rev().enumerate() {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[..s.len() - cut_index]
}

pub fn roll_slice1<T>(v: &mut [T], shift: i32) {
    let shift_usize = shift.abs().to_usize().unwrap();

    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
}

pub fn roll_slice1_replace<T>(v: &mut [T], shift: i32, replace: T) {
    roll_slice1(v, shift);
    v[v.len() - 1] = replace;
}

pub fn g_roll_slice1<'a, T>(v: &'a mut [T], shift: i32) -> &'a [T] {
    let shift_usize = shift.abs().to_usize().unwrap();

    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
    v
}

pub fn coll1_roll_replace_el<'a, C, T, V>(slice: &mut [V], shift: i32, to_replace: V) -> C
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
                .map(|(i, v)| if i <= num_need { to_replace } else { *v })
                .collect()
        }
        std::cmp::Ordering::Less => {
            let num_need = (len as i32 + shift) as usize;
            iter_
                .enumerate()
                .map(|(i, v)| if i >= num_need { to_replace } else { *v })
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

pub fn transpose_map_from_vec<K, T>(mut value: Vec<MAP<K, T>>) -> MAP<K, Vec<T>>
where
    K: Eq + Hash,
{
    let mut res = MAP::default();
    for _ in 0..value.len() {
        for (k, v) in value.remove(0) {
            match res.entry(k) {
                Entry::Vacant(entry) => {
                    let mut bind = Vec::with_capacity(value.len());
                    bind.push(v);
                    entry.insert_entry(bind);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(v);
                }
            }
        }
    }
    res
}

pub fn transpose_vec_from_map<K, T>(mut value: MAP<K, Vec<T>>) -> Vec<MAP<K, T>>
where
    K: Eq + Hash + Clone,
{
    let len = value.values().next().map(|v| v.len()).unwrap_or_default();
    let mut res = Vec::with_capacity(len);
    for _ in 0..len {
        res.push(
            value
                .iter_mut()
                .map(|v| (v.0.clone(), v.1.remove(0)))
                .collect::<MAP<_, _>>(),
        )
    }
    res
}

pub fn vec_len_sync<T: Clone>(mut src: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let min_len = src
        .iter()
        .map(|v| v.len())
        .min()
        .expect("this is nan or wtf");
    src = src
        .into_iter()
        .map(|v| v[v.len() - min_len..].to_vec())
        .collect::<Vec<Vec<T>>>();
    src
}

pub fn vec_len_sync_set<T: Clone>(src: &mut Vec<Vec<T>>) {
    let min_len = src
        .iter()
        .map(|v| v.len())
        .min()
        .expect("this is nan or wtf");
    *src = src
        .into_iter()
        .map(|v| v[v.len() - min_len..].to_vec())
        .collect::<Vec<Vec<T>>>();
}

pub fn used_procedure<C: IntoIterator<Item = T> + FromIterator<T>, T>(
    mut res: C,
    procedure: &[usize],
) -> C {
    let mut bind = res.into_iter().enumerate().collect::<Vec<(usize, T)>>();
    res = procedure
        .iter()
        .map(|i| {
            bind.remove(bind.iter().enumerate().find(|v| v.1.0 == *i).unwrap().0)
                .1
        })
        .collect();
    res
}

pub fn map_to_vec<K, T>(v: MAP_LINK<K, Vec<T>>) -> Vec<Vec<T>> {
    v.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;

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
    fn roll_slice1_replace_res_1() {
        let mut v = vec![1, 2, 3];
        roll_slice1_replace(&mut v, -1, 4);
        assert_eq_pr!(v, vec![2, 3, 4]);
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

    #[test]
    fn transpose_map_from_vec_res_1() {
        assert_eq_pr!(
            transpose_map_from_vec(vec![
                MAP::from_iter([("iter", 1)]),
                MAP::from_iter([("iter", 2)]),
            ]),
            MAP::from_iter([("iter", vec![1, 2])])
        );
    }

    #[test]
    fn transpose_vec_from_map_res_1() {
        assert_eq_pr!(
            transpose_vec_from_map(MAP::from_iter([("klines1", vec![1, 2]),])),
            vec![
                MAP::from_iter([("klines1", 1)]),
                MAP::from_iter([("klines1", 2)])
            ]
        );
    }

    #[test]
    fn used_procedure_res_1() {
        assert_eq_pr!(used_procedure(vec![0, 1, 2], &[1, 0, 2]), vec![1, 0, 2]);
    }

    #[test]
    fn vec_len_sync_res_1() {
        assert_eq!(
            vec_len_sync(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2]]),
            vec![vec![2, 3], vec![2, 3], vec![1, 2]]
        )
    }

    #[test]
    fn vec_len_sync_set_res_1() {
        let mut bind = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2]];
        vec_len_sync_set(&mut bind);
        assert_eq!(bind, vec![vec![2, 3], vec![2, 3], vec![1, 2]])
    }

    #[test]
    fn map_to_vec_res_1() {
        assert_eq_pr!(
            map_to_vec(MAP_LINK::from_iter([
                ("vec1".to_string(), vec![1]),
                ("vec2".to_string(), vec![2])
            ])),
            vec![vec![1], vec![2],]
        );
    }
}
