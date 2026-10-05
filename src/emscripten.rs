use core::{ffi::c_int, sync::atomic::AtomicU32};

unsafe extern "C" {
    fn emscripten_futex_wait(addr: *mut u32, val: u32, max_wait_ms: f64) -> c_int;
    fn emscripten_futex_wake(addr: *mut u32, count: c_int) -> c_int;
}

#[inline]
pub fn wait(a: &AtomicU32, expected: u32) {
    unsafe { emscripten_futex_wait(a.as_ptr(), expected, f64::INFINITY) };
}

#[inline]
pub fn wake_one(ptr: *const AtomicU32) {
    unsafe { emscripten_futex_wake(ptr.cast_mut().cast(), 1) };
}

#[inline]
pub fn wake_all(ptr: *const AtomicU32) {
    unsafe { emscripten_futex_wake(ptr.cast_mut().cast(), c_int::MAX) };
}
