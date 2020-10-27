mod cal;
use cal::*;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn calculate(expr: *const c_char) -> *mut c_char {
    let expr = CStr::from_ptr(expr).to_string_lossy().into_owned();
    let ret =Expr::read(expr).eval();
    CString::new(ret.into_bytes()).unwrap().into_raw()
}

#[test]
fn cal_test() {
    let ret =unsafe { CStr::from_ptr(calculate(CString::new("-6-7").unwrap().into_raw())).to_string_lossy().into_owned()};
    assert_eq!(ret[..],"-12"[..]);
}