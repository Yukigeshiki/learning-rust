use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

// sorting
fn bubble_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                // let tmp = array[j];
                // array[j] = array[j + 1];
                // array[j + 1] = tmp;
                array.swap(j, j + 1);
            }
        }
    }
}

fn merge_sort(array: &mut [i32]) {
    let middle = array.len() / 2;
    if array.len() < 2 {
        return; // No need to sort vectors with one element
    }

    let mut sorted = array.to_vec();

    merge_sort(&mut array[..middle]);
    merge_sort(&mut array[middle..]);

    merge(&array[..middle], &array[middle..], &mut sorted);

    array.copy_from_slice(&sorted); // Copy the sorted result into original vector
}

fn merge(l_arr: &[i32], r_arr: &[i32], sorted: &mut [i32]) {
    // Current loop position in left half, right half, and sorted vector
    let (mut left, mut right, mut i) = (0, 0, 0);

    while left < l_arr.len() && right < r_arr.len() {
        if l_arr[left] <= r_arr[right] {
            sorted[i] = l_arr[left];
            i += 1;
            left += 1;
        } else {
            sorted[i] = r_arr[right];
            i += 1;
            right += 1;
        }
    }

    if left < l_arr.len() {
        // If there is anything left in the left half append it after sorted members
        sorted[i..].copy_from_slice(&l_arr[left..]);
    }

    if right < r_arr.len() {
        // If there is anything left in the right half append it after sorted members
        sorted[i..].copy_from_slice(&r_arr[right..]);
    }
}

fn quick_sort(array: &mut [i32]) {
    let start = 0;
    let end = array.len() - 1;
    quick_sort_partition(array, start, end as isize);
}

fn quick_sort_partition(array: &mut [i32], start: isize, end: isize) {
    if start < end && end - start >= 1 {
        let pivot = partition(array, start as isize, end as isize);
        quick_sort_partition(array, start, pivot - 1);
        quick_sort_partition(array, pivot + 1, end);
    }
}

fn partition(array: &mut [i32], l: isize, h: isize) -> isize {
    let pivot = array[h as usize];
    let mut i = l - 1; // Index of the smaller element

    for j in l..h {
        if array[j as usize] <= pivot {
            i = i + 1;
            array.swap(i as usize, j as usize);
        }
    }

    array.swap((i + 1) as usize, h as usize);

    i + 1
}

// searching
pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut is_asc = true;
    if arr.len() > 1 {
        is_asc = arr[0] < arr[(arr.len() - 1)];
    }
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if is_asc {
            match item.cmp(&arr[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match item.cmp(&arr[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
        }
    }
    None
}
