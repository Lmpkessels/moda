/// Checks if `m` is exactly the set `{a, b, c}` (order does not matter).
///
/// Takes in three arrays of four unsigned integers each, and compares them
/// against the array `m`. Returns `true` if and only if all three arrays
/// appear in `m` and no other values are present.
fn is_set(a: [u8; 4], b: [u8; 4], c: [u8; 4], m: [[u8; 4]; 3]) -> bool {
    let mut found_a = false;
    let mut found_b = false;
    let mut found_c = false;

    for i in 0..3 {
        if m[i] == a {
            found_a = true;
        } else if m[i] == b {
            found_b = true;
        } else if m[i] == c {
            found_c = true;
        } else {
            return false; // unexpected element
        }
    }

    found_a && found_b && found_c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_true_if_a_b_and_c_are_matching_elements_in_m() {
        let a = [1, 1, 0, 0];
        let b = [1, 1, 0, 1];
        let c = [1, 0, 1, 0];
        let m = [a, b, c];
        
        assert!(is_set(a, b, c, m));
        assert!(is_set(b, c, a, m));
        assert!(is_set(c, b, a, m));
    }

    #[test]
    fn computes_false_if_a_b_and_c_are_different_than_the_elements_of_m() {
        let a = [1, 1, 0, 0];
        let b = [1, 1, 0, 1];
        let c = [1, 0, 1, 0];
        let m = [a, b, c];

        let e = [1u8; 4];
        let g = [2u8; 4];
        let f = [0u8; 4];
        
        assert!(!is_set(e, g, f, m));
        assert!(!is_set(g, f, e, m));
        assert!(!is_set(f, g, e, m));
    }
}
