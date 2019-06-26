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
        unsafe_function();
    }
}

unsafe fn unsafe_function() {}