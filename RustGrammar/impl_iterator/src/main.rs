fn main() {

}


fn num_impl_iterator(n: i32) -> impl Iterator<Item=i32> {
    (0..n).map(|x| x * 10)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_impl_iterator_success()
    {
        let num_1 = num_impl_iterator(10);
        let mut iter = num_1.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(40));
    }

}