pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[cfg(test)]
mod tests {
    use crate::helpers::vector::vec_to_array;

    #[test]
    fn vec_to_array_should_return_correct_value() {
        let vec = vec![1, 2, 3, 4, 5];

        let expected = [1, 2, 3, 4, 5];
        let actual = vec_to_array::<_, 5>(vec);

        assert_eq!(actual, expected);
    }

    #[test]
    fn vec_to_array_should_panic() {
        let vec = vec![1, 2, 3, 4, 5];
        let res = std::panic::catch_unwind(|| vec_to_array::<_, 4>(vec));

        assert!(res.is_err())
    }
}
