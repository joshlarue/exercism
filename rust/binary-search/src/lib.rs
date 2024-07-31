pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = array.len() - 1;
    println!("{}, {}", low, high);

    while low <= high {
        //if array[low] < array[0] || array[high] > array[array.len() - 1] {
        //return None;
        //}

        let mid = low + (high - low) / 2;

        if &array[mid] == &key {
            return Some(mid);
        }

        if &array[mid] < &key {
            low = mid + 1;
        } else if &array[mid] > &key && (mid as i32 - 1) >= 0 {
            high = mid - 1;
        } else {
            return None;
        }
        println!("current: low={}, mid={}, high={}", low, mid, high)
    }

    None
}
