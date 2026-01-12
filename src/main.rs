mod ex_alloc;
use ex_alloc::*;
use std::ptr::{write};
// managed to forget how to import from other
// files immideatly, tried to use import keyword...
// what a shame
fn main() {
    let my_ptr: *mut i32 = allocate::<i32>();
    // some safe way to write into allocated memory
    unsafe {
        write(my_ptr, 42);
    };
    println!("The value at our manual allocation is: {}", unsafe {
        *my_ptr
    });
    // wow, that is c now
    unsafe { *my_ptr += 10 };
    println!("Updated value: {}", unsafe { *my_ptr });
    deallocate(my_ptr as *mut u8);
    println!("Memory successfully freed.");
}
