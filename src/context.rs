use std::mem::size_of;
use libc::{size_t, c_void};
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::null;

use ffi;

pub struct TallocContext<T: Sized> {
    ptr: *mut u8,
    phantom: PhantomData<T>
}

impl<T> TallocContext<T> {
    pub fn new<U>(parent: Option<TallocContext<U>>) -> TallocContext<T> {
        let size = size_of::<T>();
        let name = CString::new("Memory context allocated from Rust.").unwrap();
        let parent_ptr = match parent {
            Some(tc) => tc.ptr,
            None => null(),
        };
        unsafe {
            let ptr = ffi::talloc_named_const(parent_ptr as *const c_void, size as size_t, name.as_ptr());
            TallocContext {
                ptr: ptr as *mut u8,
                phantom: PhantomData
            }
        }
    }
}

impl<T> Drop for TallocContext<T> {
    fn drop(&mut self) {
        let name = CString::new("Free in the Rust deallocation logic.").unwrap();
        unsafe {
            let retcode = ffi::_talloc_free(self.ptr as *mut c_void, name.as_ptr());
            if retcode != 0 {
                panic!("Failed to free memory!");
            }
        };
    }
}
