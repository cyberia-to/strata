// ---
// tags: nebu, rust
// crystal-type: source
// crystal-domain: comp
// ---
//! Multilinear polynomial evaluation over the Goldilocks field.
//!
//! A multilinear polynomial in k variables is uniquely determined by its
//! 2^k evaluations on the Boolean hypercube {0,1}^k.  Given those evaluations
//! and a point in F^k, this module computes the evaluation at that point via
//! the recursive sumcheck identity.

use crate::field::Goldilocks;

/// Evaluate a multilinear polynomial at a point.
///
/// `evals` contains the 2^k evaluations of the polynomial on the Boolean
/// hypercube in lexicographic order (index bit pattern = variable assignment,
/// LSB = last variable).
///
/// `point` contains k field coordinates in big-endian (MSB-first) order:
/// `point[0]` is the most-significant variable x_{k-1}.
///
/// The evaluation uses the recursive identity:
///
/// ```text
/// f(x_{k-1}, ..., x_0) =
///   f_L(x_{k-2}, ..., x_0)
///   + x_{k-1} * (f_R(x_{k-2}, ..., x_0) - f_L(x_{k-2}, ..., x_0))
/// ```
///
/// where f_L is the restriction to x_{k-1}=0 and f_R to x_{k-1}=1.
///
/// # Panics
///
/// Panics if `evals.len() != 2^point.len()` or if `evals` is empty.
///
/// # Allocation
///
/// Allocation-free: only stack frames proportional to `point.len()`.
pub fn multilinear_eval(evals: &[Goldilocks], point: &[Goldilocks]) -> Goldilocks {
    debug_assert_eq!(
        evals.len(),
        1usize << point.len(),
        "evals.len() must equal 2^point.len()"
    );
    debug_assert!(!evals.is_empty(), "evals must be non-empty");

    if point.is_empty() {
        return evals[0];
    }
    let mid = evals.len() / 2;
    let vl = multilinear_eval(&evals[..mid], &point[1..]);
    let vr = multilinear_eval(&evals[mid..], &point[1..]);
    vl + point[0] * (vr - vl)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(v: u64) -> Goldilocks {
        Goldilocks::new(v)
    }

    // k=1: f(x) = (1-x)*evals[0] + x*evals[1]
    // f(0) = evals[0], f(1) = evals[1]

    #[test]
    fn one_var_at_zero_returns_first_eval() {
        let evals = [g(3), g(7)];
        assert_eq!(multilinear_eval(&evals, &[g(0)]), g(3));
    }

    #[test]
    fn one_var_at_one_returns_second_eval() {
        let evals = [g(3), g(7)];
        assert_eq!(multilinear_eval(&evals, &[g(1)]), g(7));
    }

    #[test]
    fn one_var_at_midpoint_interpolates() {
        // f(0)=0, f(1)=10; at x=5: 0 + 5*(10-0) = 50
        let evals = [g(0), g(10)];
        assert_eq!(multilinear_eval(&evals, &[g(5)]), g(50));
    }

    // k=2: evals = [f(0,0), f(0,1), f(1,0), f(1,1)]
    // point = [x1, x0] (big-endian: x1 is MSV)

    #[test]
    fn two_var_at_origin() {
        // f(0,0) = 10
        let evals = [g(10), g(20), g(30), g(40)];
        assert_eq!(multilinear_eval(&evals, &[g(0), g(0)]), g(10));
    }

    #[test]
    fn two_var_at_one_zero() {
        // f(1,0) = evals[2] = 30
        let evals = [g(10), g(20), g(30), g(40)];
        assert_eq!(multilinear_eval(&evals, &[g(1), g(0)]), g(30));
    }

    #[test]
    fn two_var_at_zero_one() {
        // f(0,1) = evals[1] = 20
        let evals = [g(10), g(20), g(30), g(40)];
        assert_eq!(multilinear_eval(&evals, &[g(0), g(1)]), g(20));
    }

    #[test]
    fn two_var_at_one_one() {
        // f(1,1) = evals[3] = 40
        let evals = [g(10), g(20), g(30), g(40)];
        assert_eq!(multilinear_eval(&evals, &[g(1), g(1)]), g(40));
    }

    #[test]
    fn two_var_at_midpoint() {
        // f(x1,x0) with evals [0,0,0,4]
        // f(x1,x0) = 4*x1*x0
        // at x1=1/2, x0=1/2: 4*(1/2)*(1/2) = 1
        // In the field: 1/2 = (p+1)/2 mod p; but we can check
        // a simpler case: f(0,2) = evals[0] + 2*(evals[1]-evals[0])
        // with evals [10,20,30,40]:
        // vl = multilinear_eval([10,20], [2]) = 10 + 2*(20-10) = 30
        // vr = multilinear_eval([30,40], [2]) = 30 + 2*(40-30) = 50
        // result = 30 + 0*(50-30) = 30 (x1=0)
        let evals = [g(10), g(20), g(30), g(40)];
        let result = multilinear_eval(&evals, &[g(0), g(2)]);
        // vl = 10+2*10=30; vr = 30+2*10=50; out = 30 + 0*(50-30) = 30
        assert_eq!(result, g(30));
    }
}
