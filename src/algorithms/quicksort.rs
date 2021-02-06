use std::convert::TryInto;

pub struct QuickSort { }

#[allow(dead_code)]
#[allow(unused_comparisons)]
impl QuickSort {
    pub fn sort(arr: &mut Vec<u32>) {
        let mut b = 0;
        let mut t = (arr.len() - 1) as isize;
        QuickSort::create(arr, &mut b, &mut t);
    }

    fn partition(arr: &mut Vec<u32>, left: &mut isize, right: &mut isize) -> isize {
        let pivot = arr[*right as usize];
        let parser = *left as isize;
        let mut low_index = parser - 1;
        let temp_left = left;
        let temp_right = right;
        for iter in *temp_left..*temp_right {
            if arr[iter as usize] <= pivot {
                low_index += 1;
                let temp = arr[low_index as usize];
                arr[low_index as usize] = arr[iter as usize];
                arr[iter as usize] = temp;
            }
        }

        let temp1 = arr[(low_index + 1) as usize];
        let sub_right = *(temp_right) as usize;
        arr[(low_index + 1) as usize] = arr[sub_right];
        arr[sub_right] = temp1;

        low_index + 1
    }

    fn create(arr: &mut Vec<u32>, left: &mut isize, right: &mut isize) {
        if left < right {
            let partition_index = QuickSort::partition(arr, left, right);
            let partition_index: isize = (partition_index as usize).try_into().unwrap();
            QuickSort::create(arr, left, &mut (partition_index - 1));
            QuickSort::create(arr, &mut (partition_index + 1), right);
        }
    }
}