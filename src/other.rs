use std::borrow::Borrow;

use num_traits::ToPrimitive;


pub fn lstrip(
    s: &'static str,
    cut_before: char,
) -> &'static str {
    let mut cut_index: usize = 0;

    for c in s
        .chars()
        .enumerate() 
    {
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

    for c in s
        .chars()
        .rev()
        .enumerate() 
    {
        if c.1 == cut_before {
            cut_index = c.0;
            break;
        }
    }
    &s[..s.len() - cut_index]
}

pub fn roll_slice1<T>(
    v: &mut [T],
    shift: &i8,
)
{    
    let shift_usize = (*shift)
        .abs()
        .to_usize()
        .unwrap();
    
    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
}

pub fn g_roll_slice1<'a, T>(
    v: &'a mut [T],
    shift: &i8,
) -> &'a [T]
{    
    let shift_usize = (*shift)
        .abs()
        .to_usize()
        .unwrap();
    
    match shift.cmp(&0) {
        std::cmp::Ordering::Greater => v.rotate_right(shift_usize),
        std::cmp::Ordering::Less => v.rotate_left(shift_usize),
        std::cmp::Ordering::Equal => {}
    }
    v
}

pub fn coll1_roll_replace_el<'a, C, T, V,>(
    slice: &mut [V],
    shift: &i8,
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
    let shift_usize = (*shift)
        .abs()
        .to_usize()
        .unwrap();

    match shift.cmp(&0){
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
            let num_need = (len as i8 + shift) as usize;
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
        std::cmp::Ordering::Equal => iter_.copied().collect()
    }
}
