pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = (left + right) / 2;
        
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}
