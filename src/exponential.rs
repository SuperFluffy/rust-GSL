//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use ffi;
use std::mem::zeroed;
use enums;

/// This routine provides an exponential function \exp(x) using GSL semantics and error checking.
pub fn exp(x: f64) -> f64 {
    unsafe { ffi::gsl_sf_exp(x) }
}

/// This routine provides an exponential function \exp(x) using GSL semantics and error checking.
pub fn exp_e(x: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exp_e(x, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This function computes the exponential \exp(x) using the gsl_sf_result_e10 type to return a result with extended range.
/// This function may be useful if the value of \exp(x) would overflow the numeric range of double.
pub fn exp_e10_e(x: f64) -> (enums::Value, ::types::ResultE10) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result_e10>() };
    let ret = unsafe { ffi::gsl_sf_exp_e10_e(x, &mut result) };

    (enums::Value::from(ret), ::types::ResultE10{val: result.val, err: result.err, e10: result.e10})
}

/// This routine exponentiates x and multiply by the factor y to return the product y \exp(x).
pub fn exp_mult(x: f64, y: f64) -> f64 {
    unsafe { ffi::gsl_sf_exp_mult(x, y) }
}

/// This routine exponentiates x and multiply by the factor y to return the product y \exp(x).
pub fn exp_mult_e(x: f64, y: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exp_mult_e(x, y, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This function computes the exponential \exp(x) using the gsl_sf_result_e10 type to return a result with extended range.
/// This function may be useful if the value of \exp(x) would overflow the numeric range of double.
pub fn exp_mult_e10_e(x: f64, y: f64) -> (enums::Value, ::types::ResultE10) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result_e10>() };
    let ret = unsafe { ffi::gsl_sf_exp_mult_e10_e(x, y, &mut result) };

    (enums::Value::from(ret), ::types::ResultE10{val: result.val, err: result.err, e10: result.e10})
}

/// This routine computes the quantity \exp(x)-1 using an algorithm that is accurate for small x.
pub fn expm1(x: f64) -> f64 {
    unsafe { ffi::gsl_sf_expm1(x) }
}

/// This routine computes the quantity \exp(x)-1 using an algorithm that is accurate for small x.
pub fn expm1_e(x: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_expm1_e(x, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This routine computes the quantity (\exp(x)-1)/x using an algorithm that is accurate for small x.
/// For small x the algorithm is based on the expansion (\exp(x)-1)/x = 1 + x/2 + x^2/(2*3) + x^3/(2*3*4) + \dots.
pub fn exprel(x: f64) -> f64 {
    unsafe { ffi::gsl_sf_exprel(x) }
}

/// This routine computes the quantity (\exp(x)-1)/x using an algorithm that is accurate for small x.
/// For small x the algorithm is based on the expansion (\exp(x)-1)/x = 1 + x/2 + x^2/(2*3) + x^3/(2*3*4) + \dots.
pub fn exprel_e(x: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exprel_e(x, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This routine computes the quantity 2(\exp(x)-1-x)/x^2 using an algorithm that is accurate for small x.
/// For small x the algorithm is based on the expansion 2(\exp(x)-1-x)/x^2 = 1 + x/3 + x^2/(3*4) + x^3/(3*4*5) + \dots.
pub fn exprel_2(x: f64) -> f64 {
    unsafe { ffi::gsl_sf_exprel_2(x) }
}

/// This routine computes the quantity 2(\exp(x)-1-x)/x^2 using an algorithm that is accurate for small x.
/// For small x the algorithm is based on the expansion 2(\exp(x)-1-x)/x^2 = 1 + x/3 + x^2/(3*4) + x^3/(3*4*5) + \dots.
pub fn exprel_2_e(x: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exprel_2_e(x, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This routine computes the N-relative exponential, which is the n-th generalization of the functions gsl_sf_exprel and gsl_sf_exprel_2.
/// The N-relative exponential is given by:
///
/// ```latex
/// exprel_N(x) = N!/x^N (\exp(x) - \sum_{k=0}^{N-1} x^k/k!)
///
///             = 1 + x/(N+1) + x^2/((N+1)(N+2)) + ...
///
///             = 1F1 (1,1+N,x)
/// ```
pub fn exprel_n(n: i32, x: f64) -> f64 {
    unsafe { ffi::gsl_sf_exprel_n(n, x) }
}

/// This routine computes the N-relative exponential, which is the n-th generalization of the functions gsl_sf_exprel and gsl_sf_exprel_2.
/// The N-relative exponential is given by:
///
/// ```latex
/// exprel_N(x) = N!/x^N (\exp(x) - \sum_{k=0}^{N-1} x^k/k!)
///
///             = 1 + x/(N+1) + x^2/((N+1)(N+2)) + ...
///
///             = 1F1 (1,1+N,x)
/// ```
pub fn exprel_n_e(n: i32, x: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exprel_n_e(n, x, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This function exponentiates x with an associated absolute error dx.
pub fn exp_err_e(x: f64, dx: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exp_err_e(x, dx, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This function exponentiates a quantity x with an associated absolute error dx using the ::types::ResultE10 type to return a result with extended range.
pub fn exp_err_e10_e(x: f64, dx: f64) -> (enums::Value, ::types::ResultE10) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result_e10>() };
    let ret = unsafe { ffi::gsl_sf_exp_err_e10_e(x, dx, &mut result) };

    (enums::Value::from(ret), ::types::ResultE10{val: result.val, err: result.err, e10: result.e10})
}

/// This routine computes the product y \exp(x) for the quantities x, y with associated absolute errors dx, dy.
pub fn exp_mult_err_e(x: f64, dx: f64, y: f64, dy: f64) -> (enums::Value, ::types::Result) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
    let ret = unsafe { ffi::gsl_sf_exp_mult_err_e(x, dx, y, dy, &mut result) };

    (enums::Value::from(ret), ::types::Result{val: result.val, err: result.err})
}

/// This routine computes the product y \exp(x) for the quantities x, y with associated absolute errors dx, dy using the gsl_sf_result_e10 type to return a result with extended range.
pub fn exp_mult_err_e10_e(x: f64, dx: f64, y: f64, dy: f64) -> (enums::Value, ::types::ResultE10) {
    let mut result = unsafe { zeroed::<ffi::gsl_sf_result_e10>() };
    let ret = unsafe { ffi::gsl_sf_exp_mult_err_e10_e(x, dx, y, dy, &mut result) };

    (enums::Value::from(ret), ::types::ResultE10{val: result.val, err: result.err, e10: result.e10})
}
