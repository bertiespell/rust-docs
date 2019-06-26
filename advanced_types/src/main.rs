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