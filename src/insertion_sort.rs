pub fn insertion_sort(mut arr: [i32; 6]) -> [i32; 6] {
    for i in 0..6 {
        let mut j = i;
        while j > 0 && arr[j-1] > arr[j] {
            let left = arr[j-1];
            let right = arr[j];

            arr[j-1] = right;
            arr[j] = left;
            j -= 1;
        }
    }
    arr
}

#[cfg(test)]
mod test {
    use super::insertion_sort;

    #[test]
    fn sort() {
        let unsorted_arr = [2,8,5,3,9,4];

        assert_eq!([2,3,4,5,8,9], insertion_sort(unsorted_arr));
    }
}