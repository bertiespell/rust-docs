// The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the Drop trait on any type, and the code you specify can be used to release resources like files or network connections. We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For example, Box<T> customizes Drop to deallocate the space on the heap that the box points to.

// Specify the code to run when a value goes out of scope by implementing the Drop trait. The Drop trait requires you to implement one method named drop that takes a mutable reference to self. 
fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created."); // When main finishes - these will go out of scope and be dropped - printing out the line below declared in the trait... Variables are dropped in the reverse order of their creation, so d was dropped before c. 

    // Dropping early - included in the prelude

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping: {}", self.data);
    }
}

// ====== Calling Drop Early ======

// Occasionally, however, you might want to clean up a value early. One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock to run so other code in the same scope can acquire the lock.

// Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

// Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main. This would be a double free error because Rust would be trying to clean up the same value twice.

