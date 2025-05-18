pub fn binary_search<T: Ord>(array: &[T], target: &T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if &array[mid] == target {
            return Some(mid);
        } else if &array[mid] < target {
            low = mid + 1;
        } else {
            if mid == 0 { break; } // prevent underflow
            high = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let arr = vec![1, 2, 3];
        assert_eq!(binary_search(&arr, &2), Some(1));
    }
}