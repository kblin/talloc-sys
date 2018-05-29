extern crate libc;

mod ffi;
mod context;

use context::TallocContext;

pub fn talloc<U, T>(ctx: Option<TallocContext<U>>) -> TallocContext<T> {
    TallocContext::<T>::new(ctx)
}
