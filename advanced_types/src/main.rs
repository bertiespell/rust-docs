/**
 * Another use of the newtype pattern is in abstracting away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type if we used the new type directly to restrict the available functionality, for example.
 * 
 * Newtypes can also hide internal implementation. For example, we could provide a People type to wrap a HashMap<i32, String> that stores a person’s ID associated with their name. Code using People would only interact with the public API we provide, such as a method to add a name string to the People collection; that code wouldn’t need to know that we assign an i32 ID to names internally. The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details
 */
fn main() {
    use_type_alias();
}

// Creating Type Synonyms with Type Aliases

// Along with the newtype pattern, Rust provides the ability to declare a type alias to give an existing type another name. For this we use the type keyword. For example, we can create the alias Kilometers to i32 like so:

type Kilometers = i32;

fn use_type_alias() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:
// Box<dyn Fn() + Send + 'static>

// Makes writing things less time consuming and error prone

type Thunk = Box<dyn Fn() + Send + 'static>;

fn example() {
    let f: Thunk = Box::new(|| println!("hi"));
}

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    unimplemented!();
}

// THE NEVER TYPE

// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values. We prefer to call it the never type because it stands in the place of the return type when a function will never return.

// This code is read as “the function bar returns never.” Functions that return never are called diverging functions. We can’t create values of the type ! so bar can never possibly return.
fn bar() -> ! {
    // --snip--
}

// we discussed that match arms must all return the same type. So, for example, the following code doesn’t work:

fn doesnt_work() {
    // let guess = match guess.trim().parse() {
    //     Ok(_) => 5,
    //     Err(_) => "hello",
    // }
}

fn but_this_does() {
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, // The type of guess in this code would have to be an integer and a string, and Rust requires that guess have only one type. So what does continue return? How were we allowed to return a u32 from one arm and have another arm that ends with continue in Listing 19-34?

        // As you might have guessed, continue has a ! value. That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value. Because ! can never have a value, Rust decides that the type of guess is u32.

        // The formal way of describing this behavior is that expressions of type ! can be coerced into any other type. We’re allowed to end this match arm with continue because continue doesn’t return a value; instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.
    };
}

// The never type is useful with the panic! macro as well. Remember the unwrap function that we call on Option<T> values to produce a value or panic? Here is its definition:

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

// One final expression that has the type ! is a loop:

fn forever() {
    print!("forever ");

    loop {
        print!("and ever "); // Here, the loop never ends, so ! is the value of the expression. However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.
    }
}

// ~~~~~~~~ Dynamically Sized Types and the Sized Trait ~~~~~~~~~

// Due to Rust’s need to know certain details, such as how much space to allocate for a value of a particular type, there is a corner of its type system that can be confusing: the concept of dynamically sized types. Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.

// Let’s dig into the details of a dynamically sized type called str, which we’ve been using throughout the book. That’s right, not &str, but str on its own, is a DST. We can’t know how long the string is until runtime, meaning we can’t create a variable of type str, nor can we take an argument of type str. Consider the following code, which does not work:

fn doesnt_work_again() {
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory. If Rust allowed us to write this code, these two str values would need to take up the same amount of space. But they have different lengths: s1 needs 12 bytes of storage and s2 needs 15. This is why it’s not possible to create a variable holding a dynamically sized type.

    // So what do we do? In this case, you already know the answer: we make the types of s1 and s2 a &str rather than a str. => the slice data structure stores the starting position and the length of the slice.
}

/**
 * So although a &T is a single value that stores the memory address of where the T is located, a &str is two values: the address of the str and its length. As such, we can know the size of a &str value at compile time: it’s twice the length of a usize. That is, we always know the size of a &str, no matter how long the string it refers to is. In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
 * 
 * We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>. In fact, you’ve seen this before but with a different dynamically sized type: traits. Every trait is a dynamically sized type we can refer to by using the name of the trait. In Chapter 17 in the “Using Trait Objects that Allow for Values of Different Types” section, we mentioned that to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).

To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:

fn generic<T>(t: T) {
    // --snip--
}

is actually treated as though we had written this:

fn generic<T: Sized>(t: T) {
    // --snip--
}

By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

A trait bound on ?Sized is the opposite of a trait bound on Sized: we would read this as “T may or may not be Sized.” This syntax is only available for Sized, not any other traits.

Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.
 */