// TODO: Allow to dynamically change this. For example with a --verbose flag
#[cfg(debug_assertions)]
const DBG: bool = false; // Usually it's too verbose even for debugging
#[cfg(not(debug_assertions))]
const DBG: bool = false;

pub fn is_debug() -> bool {
    DBG
}

/// Convert any type to a u8 slice (Like a C byte buffer)
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    let len = ::std::mem::size_of::<T>();
    ::std::slice::from_raw_parts((p as *const T) as *const u8, len)
}

pub unsafe fn any_vec_as_u8_slice<T: Sized>(p: &Vec<T>) -> &[u8] {
    let len = ::std::mem::size_of::<T>() * p.len();
    ::std::slice::from_raw_parts((p.as_ptr() as *const T) as *const u8, len)
}

pub fn print_buffer(buffer: &[u8]) {
    for byte in buffer {
        print!("{:#X} ", byte);
    }
    println!();
}
