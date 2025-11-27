// Modular wrap around, reduces the number till the final remainder remains.
fn mod_reduce(mut n: u128, m: u128) -> u128 {
    assert!(m != 0, "Modulus cannot be 0!");

    while n >= m {
        n -= m
    }

    n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3_digit_integers() {
        let n = 121;
        let m = 111;

        let result = mod_reduce(n, m);
        let expected = n % m;

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_10_digit_number() {
        let n = 1341812313;
        let m = 2932498128;

        let result = mod_reduce(n, m);
        let expected = n % m;

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_give_back_n_since_n_less_then_m() {
        let n = 12111;
        let m = 54222;

        let result = mod_reduce(n, m);
        let expected = n;

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_multiple_values_where_n_fits_into_m() {
        let n = [121, 11, 31414, 41633, 999];
        let m = 154413511;

        
        for i in n {
            let result = mod_reduce(i, m);
            let expected = i % m;
            
            assert_eq!((result), (expected));
        }
    }
}