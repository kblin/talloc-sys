use libc::{size_t, c_int, c_char, c_void};

#[link(name="talloc")]
extern {
    pub fn talloc_named_const(context: *const c_void, size: size_t, name: *const c_char) -> *mut c_void;
    pub fn _talloc_free(context: *mut c_void, location: *const c_char) -> c_int;
}
