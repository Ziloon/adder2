pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    fn swap(a: &mut i32, b: &mut i32) {
        *a ^= *b;
        *b ^= *a;
        *a ^= *b;
    }

    #[test]
    fn it_works() {
        let result = add(3, 5);
        assert_eq!(result, 8);
    }

    #[test]
    fn swap_ok() {
        let mut a = 4;
        let mut b = 5;
        swap(&mut a, &mut b);
        assert_eq!(a, 5);
        assert_eq!(b, 4);
    }
}
