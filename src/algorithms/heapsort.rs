pub struct HeapSort { }

#[allow(dead_code)]
#[allow(unused_comparisons)]
impl HeapSort {
    fn create() -> Self {
        Self { }
    }
    
    pub fn sort(arr: &mut Vec<u32>) {
        let length = arr.len();
        for iter in (0..(length / 2) - 1).rev() {
            HeapSort::create().new(arr, length, iter);
        }

        for iter in (0..(length)).rev() {
            let temp = arr[0];
            arr[0] = arr[iter];
            arr[iter] = temp;
            HeapSort::create().new(arr, iter, 0);
        }
    }


    fn new(self, arr: &mut Vec<u32>, length: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        if left < length && arr[left] > arr[largest] {
            largest = left;
        }
        if right < length && arr[right] > arr[largest] {
            largest = right;
        }
        if largest != i {
            let swap = arr[i];
            arr[i] = arr[largest];
            arr[largest] = swap;
            self.new(arr, length, largest);
        }
    } 
}