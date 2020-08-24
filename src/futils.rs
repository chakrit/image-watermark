use std::cmp::Ordering;
use std::iter::Iterator;

// max_f32 compares two floating point numbers and try it's best to not return an unusable value
pub fn max_f32(l: f32, r: f32) -> f32 {
    max_f32_opt(l, r).unwrap_or_default()
}
fn max_f32_opt(l: f32, r: f32) -> Option<f32> {
    match l.partial_cmp(&r) {
        Some(Ordering::Greater) => Some(l),
        Some(_) => Some(r),
        None => {
            if l.is_finite() {
                Some(l)
            } else if r.is_finite() {
                Some(r)
            } else {
                None
            }
        }
    }
}

// min_f32 compares two floating point numbers and try it's best to not return an unusable value.
pub fn min_f32(l: f32, r: f32) -> f32 {
    min_f32_opt(l, r).unwrap_or_default()
}
fn min_f32_opt(l: f32, r: f32) -> Option<f32> {
    match l.partial_cmp(&r) {
        Some(Ordering::Less) => Some(l),
        Some(_) => Some(r),
        None => {
            if l.is_finite() {
                Some(l)
            } else if r.is_finite() {
                Some(r)
            } else {
                None
            }
        }
    }
}

pub trait F32Extension {
    fn max_f32(self) -> Option<f32>;
    fn min_f32(self) -> Option<f32>;
    fn sum_f32(self) -> f32;
}

impl<T> F32Extension for T
where
    T: Iterator<Item = f32>,
{
    fn max_f32(self) -> Option<T::Item> {
        self.fold(None, |acc: Option<f32>, item: f32| {
            if !item.is_normal() {
                return acc;
            }
            match acc {
                None => Some(item),
                Some(cur) => max_f32_opt(cur, item),
            }
        })
    }

    fn min_f32(self) -> Option<T::Item> {
        self.fold(None, |acc: Option<f32>, item: f32| {
            if !item.is_normal() {
                return acc;
            }
            match acc {
                None => Some(item),
                Some(cur) => min_f32_opt(cur, item),
            }
        })
    }

    fn sum_f32(self) -> f32 {
        self.fold(0.0, |acc: f32, item: f32| {
            if !item.is_normal() {
                return acc;
            }

            acc + item
        })
    }
}
