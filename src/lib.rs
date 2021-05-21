#[no_mangle]
#[inline(never)]
#[cfg(target_arch = "wasm32")]
pub fn f() -> u64 {
    unsafe { host() }
    92
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn host();
}
