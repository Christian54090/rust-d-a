pub fn binary_search(vec:Vec<i32>, elem:i32) -> bool {
    let vec_mid = vec.len()/2;

    let mut left = vec[0..vec_mid].to_vec();
    while left[left.len()-1] < elem && left.len() > 1 {
        let length = left.len() as f32;
        let mid:f32 = length/2.0;
        let floor = mid.floor() as usize;
        left = left[0..floor].to_vec();
    }

    let mut right = vec[vec_mid..vec.len()-1].to_vec();
    while right[0] > elem && right.len() > 1 {
        let length = right.len() as f32;
        let mid:f32 = length/2.0;
        let floor = mid.floor() as usize;
        right = right[floor..right.len()-1].to_vec();
    }

    vec[vec_mid] == elem || left[left.len()] == elem || right[0] == elem
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn is_true() {
        let vec = vec![1,3,4,5,13,20,25,40,42,44,53];
        assert_eq!(true, binary_search(vec, 5));
    }

    fn is_false() {
        let vec = vec![1,3,4,5,13,20,25,40,42,44,53];
        assert_eq!(false, binary_search(vec, 43));
    }
}