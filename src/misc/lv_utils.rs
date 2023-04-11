use std::cmp::Ordering;

#[no_mangle]
pub unsafe extern "C" fn _lv_utils_bsearch(
    key: *const std::ffi::c_void,
    base: *const std::ffi::c_void,
    n: u32,
    size: u32,
    cmp: fn(*const std::ffi::c_void, *const std::ffi::c_void) -> i32,
) -> *const std::ffi::c_void {
    let mut left = 0;
    let mut right = n - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        let ptr = unsafe { base.offset((mid * size) as isize) };
        match cmp(key, ptr) {
            i if i < 0 => right = mid - 1,
            i if i > 0 => left = mid + 1,
            _ => return ptr,
        }
    }

    std::ptr::null()
}

