/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn qsort<T: std::cmp::PartialOrd>(array: &mut [T], low: usize, high: usize) {
    let pivot = low;
    let mut left = low;
    let mut right = high;
    println!("{}, {}", low, high);
    while left < right {
        while left < right && array[right] > array[pivot] {
            right -= 1;
        }
        while left < right && array[left] <= array[pivot] {
            left += 1;
        }
        if left != right {
            array.swap(left, right);
        }
    }
    array.swap(pivot, left);
    if low < left {
        qsort(array, low, left);
    }
    if left + 1 < high {
        qsort(array, left + 1, high);
    }
}

fn sort<T: std::cmp::PartialOrd + std::cmp::Ord>(array: &mut [T]) {
    // array.sort();
    qsort(array, 0, array.len() - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
