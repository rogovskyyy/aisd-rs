use std::cmp::Ordering;
use std::cmp;

pub struct QuickSortV2 { }

impl QuickSortV2 {

    #[inline]
    pub fn sort<T: Ord>(vec: &mut [T]) {
        QuickSortV2::quicksort_by(vec, &|x, y| x.cmp(y))
    }

    pub fn quicksort_by<T, F>(vec: &mut [T], cmp: &F)
        where F: Fn(&T, &T) -> Ordering {

        let len: usize = vec.len();
        if len <= 1 {
            return;
        }

        let pivot: usize = 0;
        vec.swap(pivot, len / 2);

        let mut left: usize = 1;
        let mut right: usize = vec.len() - 1;

        loop {
            while left < len && cmp(&vec[left], &vec[pivot]) != Ordering::Greater {
                left += 1
            }
            while right > 0 && cmp(&vec[right], &vec[pivot]) != Ordering::Less {
                right -= 1
            }
            if left >= right {
                break;
            }
            vec.swap(left, right);
            left += 1;
            right -= 1;
        }
        vec.swap(pivot, right);
        QuickSortV2::quicksort_by(&mut vec[0..cmp::min(left - 1, right)], cmp);
        QuickSortV2::quicksort_by(&mut vec[cmp::max(left, right + 1)..], cmp);
    }
}