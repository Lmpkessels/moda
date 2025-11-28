// Modular wrap around, reduces the number till the final remainder remains.
fn mod_reduce(mut n: i128, m: i128) -> i128 {
    assert!(m != 0);

    while n < 0 {
        n += m
    }

    while n >= m {
        n -= m
    }

    n
}

// Modular addition applied with the simple addition operator.
fn mod_add(a: i128, b: i128, m: i128) -> i128 {
    mod_reduce(a + b, m)
}

// Modular subtraction applied with the simple addition operator.
fn mod_sub(a: i128, b: i128, m: i128) -> i128 {
    mod_reduce(a - b, m)
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

    #[test]
    fn test_mod_add_with_multiple_values() {
        let n = [1211, 11211, 9812, 1213, 9999, 19341];
        let m = 2109739148;

        for i in 0..n.len() {
            for j in i + 1..n.len() {
                let result = mod_add(n[i], n[j], m);
                let expected = (n[i] + n[j]) % m;

                assert_eq!((result), (expected));
            }
        }
    }

    #[test]
    fn test_mod_sub_with_multiple_values() {
        let n = [7777, 1111, 666891, 101112, 12331333, 181920];
        let m = 3917484;

        for i in 0..n.len() {
            for j in i + 1..n.len() {
                let result = mod_sub(n[i], n[j], m);
                let expected = ((n[i] - n[j]) % m + m) % m;
                
                assert_eq!((result), (expected));
            }
        }
    }
}