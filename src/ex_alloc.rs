use std::alloc::{Layout, alloc, dealloc};
// looks like very experimental module for me
// usually those should be abstracted by Box, Vec, Arc
// Arc - some struct for threads, Atomically Reference Counted
// but lets try

pub fn allocate<T>() -> *mut T {
    // the whole thing should be wrapped in unsafe
    let layout = Layout::new::<T>();

    let ptr = unsafe { alloc(layout) } as *mut T;

    if ptr.is_null() {
        panic!("Memory allocation failed, layout: {:?}", layout);
    } // reasonable

    ptr
}

pub fn deallocate<T>(ptr: *mut T) {
    // the whole thing should be wrapped in unsafe
    // nvm, local unsafe works
    let layout = Layout::new::<T>();
    unsafe { dealloc(ptr as *mut u8, layout) };
}
