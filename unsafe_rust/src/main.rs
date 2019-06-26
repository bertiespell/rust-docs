// Why unsafe Rust?
/**

Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe. If Rust didn’t let you do unsafe operations, you couldn’t do certain tasks. Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even writing your own operating system. Working with low-level systems programming is one of the goals of the language.

unsafe {
    // Lets you do four things you can't normally: 

    1. Dereference a raw pointer
    2. Call an unsafe function or method
    3. Access or modify a mutable static variable
    4. Implement an unsafe trait    
}

Note: unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked

To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API
 */
fn main() {
    use_raw_pointer();
    use_split_at_mut();
}

/**

1. Dereferencing a Raw Pointer

Unsafe Rust has two new types called raw pointers that are similar to references. As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively. The asterisk isn’t the dereference operator; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

a. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
b. Aren’t guaranteed to point to valid memory
c. Are allowed to be null
d. Don’t implement any automatic cleanup
 */

fn use_raw_pointer() {
    // Notice that we don’t include the unsafe keyword in this code. We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block
    let mut num = 5;
    let r1 = &num as *const i32; // note the * const
    let r2 = &mut num as *mut i32; // note the *mut
    // If we instead tried to create an immutable and a mutable reference to num, the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references. With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race

    unsafe { // dereferencing must be in an unsafe block
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // We’ve created raw pointers by using as to cast an immutable and a mutable reference into their corresponding raw pointer types. Because we created them directly from references guaranteed to be valid, we know these particular raw pointers are valid, but we can’t make that assumption about just any raw pointer.

    // Next, we’ll create a raw pointer whose validity we can’t be so certain of. Trying to use arbitrary memory is undefined: there might be data at that address or there might not, the compiler might optimize the code so there is no memory access, or the program might error with a segmentation fault. Usually, there is no good reason to write code like this, but it is possible: 

    let address = 0x012345usize;
    let r = address as *const i32;
}

// With all of these dangers, why would you ever use raw pointers? One major use case is when interfacing with C code, as you’ll see in the next section, “Calling an Unsafe Function or Method.” Another case is when building up safe abstractions that the borrow checker doesn’t understand. 

fn call_unsafe_functions() {
    unsafe {
        unsafe_function(); // Bodies of unsafe functions are effectively unsafe blocks, so to perform other unsafe operations within an unsafe function, we don’t need to add another unsafe block.
    }
}

unsafe fn unsafe_function() {}

// Creating a safe abstraction around unsafe code
// Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe. In fact, wrapping unsafe code in a safe function is a common abstraction. As an example, let’s study a function from the standard library, split_at_mut, that requires some unsafe code and explore how we might implement it. This safe method is defined on mutable slices: it takes one slice and makes it two by splitting the slice at the index given as an argument.

fn use_split_at_mut() {
    let mut v = vec!(1, 2, 3, 4, 5, 6);

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;

fn example_impl_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    // (&mut slice[0..mid], &mut slice[mid..]) // won't compile because second mutable reference occurs here

    // Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice. Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.

    let ptr = slice.as_mut_ptr();

    // The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and must trust that this pointer is valid. The offset method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer. Therefore, we had to put an unsafe block around our calls to slice::from_raw_parts_mut and offset so we could call them. By looking at the code and by adding the assertion that mid must be less than or equal to len, we can tell that all the raw pointers used within the unsafe block will be valid pointers to data within the slice. This is an acceptable and appropriate use of unsafe.
    unsafe {
        // the slice::from_raw_parts_mut function takes a raw pointer and a length, and it creates a slice. We use this function to create a slice that starts from ptr and is mid items long. Then we call the offset method on ptr with mid as an argument to get a raw pointer that starts at mid, and we create a slice using that pointer and the remaining number of items after mid as the length.
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}