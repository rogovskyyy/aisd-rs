pub struct ShellSort { }

impl ShellSort  {
    pub fn sort(arr: &mut Vec<u32>) {
        let mut inc = 3;
        while inc > 0 {
            for iter in 0..arr.len() {
                let mut j = iter;
                let temp = arr[iter];
                while (j >= inc) && (arr[j - inc] > temp) {
                    arr[j] = arr[j - inc];
                    j = j - inc;
                }
                arr[j] = temp;
            }
            if inc / 2 != 0 {
                inc = inc / 2;
            } else if inc == 1 {
                inc = 0;
            } else {
                inc = 1;
            }
        }
    }
}