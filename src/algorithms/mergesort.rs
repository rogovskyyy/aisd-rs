pub struct MergeSort { }


#[allow(unused_variables)]
impl MergeSort {
    pub fn sort(arr: &mut Vec<u32>) {
        let exec = MergeSort::merge_sort(arr);
        for iter in 0..arr.len() {
            arr[iter] = exec[iter];
        }
    }

    fn merge_sort(arr: &mut Vec<u32>) -> Vec<u32> {
        let (mut left, mut right): (Vec<u32>, Vec<u32>);
        if arr.len() <= 1 {
            return arr.to_vec();
        }
        let mid_point = arr.len() / 2;
        left = Vec::with_capacity(mid_point);
        if arr.len() % 2 == 0 {
            right = Vec::with_capacity(mid_point);
        } else {
            right = Vec::with_capacity(mid_point + 1);
        }
        for iter in 0..mid_point {
            left.push(arr[iter]);
        }
        for iter in mid_point..arr.len() {
            right.push(arr[iter]);
        }

        left = MergeSort::merge_sort(&mut left).to_vec();
        right = MergeSort::merge_sort(&mut right).to_vec();
        MergeSort::merge(&mut left, &mut right)
    }

    fn merge(left: &mut Vec<u32>, right: &mut Vec<u32>) -> Vec<u32> {
        let result_length = right.len() + left.len();
        let mut result: Vec<u32> = vec![0; result_length];
        let (mut index_left, mut index_right, mut index_result) = (0, 0, 0);
        while index_left < left.len() || index_right < right.len() {
            if index_left < left.len() && index_right < right.len() {
                if left[index_left] <= right[index_right] {
                    result[index_result] = left[index_left];
                    index_left += 1;
                    index_result += 1;
                } else {
                    result[index_result] = right[index_right];
                    index_right += 1;
                    index_result += 1;
                }
            } else if index_left < left.len() {
                result[index_result] = left[index_left];
                index_left += 1;
                index_result += 1;
            } else if index_right < right.len() { 
                result[index_result] = right[index_right];
                index_right += 1;
                index_result += 1;
            }
        }
        result
    }
}