fn main() {
    println!("Hello, world!");
    let width1 = 30;
    let heigth1 = 50;

    println!("The area of the rectangle is {}", area(width1, heigth1));

    let rec = Rectangle {
        width: width1,
        height: heigth1
    };

    let area = struct_refactor(&rec);
    println!("The area of the Struct Rectangle is {}", area);
    // the fact that rec is borrowed means we can use it down here

    println!("{}", rec.width); // i.e. this will still work.
    // if we had declared struct_refactor to take ownership (by accepting the Struct) - then it would have fallen out of scope at the end of the function and we couldn't use it here!

    print_struct(rec);

    test_impl();

    test_squares();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// we can do better than area though because this doesn't represent the fact that the two parameters are related (they make up a square)

// refactor with tuples

fn tuple_refactor(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// this isn't necessarily better though since tuples don't name their elements
// What if it mattered that we mixed up width and height (other area calculations - other than a square)

// refactor with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// takes an immutable borrow of a Rectangle instance
fn struct_refactor(rec: &Rectangle) -> u32 { // for this we only want to BORROW the struct, RATHER THAN take ownership
    // so instead of rec: Rectangle we have rec: &Rectangle
    rec.height * rec.width
}

fn print_struct(rec: Rectangle) {
    // you can't println!("{}", rec); because it doesn't implement display
    // another example {:?}
    println!("{:#?}", rec);
}

// this (refactored) area calculator should really only be used on the Square! Let's make it so that this is a method on that struct

// methods are similar to functions - use fn syntax
// BUT they are defined within the context of a struct (or an enum or trait)
// Their first argument is always self

impl Rectangle {
    fn area(&self) -> u32 { // we still need the & here
    // methods can still:
        // take ownership of self
        // borrow self immutably
        // borrow self mutably
    // (Just as they can with any other type of parameter)
        self.width * self.height

        // here we use &self because we only want to borrow self
        // we want to read data, not write it
        // if we wanted to change the instance on which we've called the method on we would need &mut self
        // having a method that takes ownership of the instance by just calling self - is RARE - it is usually employed when the caller wants to transform self into something else and you want to prevent the caller from using the original instance after the transformation.
    }
}

fn test_impl() {
    let square = Rectangle {
        width: 11,
        height:9
    };
    println!("The area of our cute little square {}", square.area());
}

// Rust has automatic referencing and dereferencing

// In other languages like C and C++ there is a difference between accessing methods on an actual instance and accessing methods on a points
// -> for a pointer (this deferences the pointer first)
// . for an instance

// e.g. is object is a pointer
// object->doSomething() is similar to (*object).doSomething()

// Rust doesn't have an equivalent to ->
// because it uses AUTOMATIC REFERENCING AND DEREFERENCING
// Calling methods in Rust is one of the places that has this behaviour

// So...

// When you call a method with object.something(), Rust automatically adds in & or &mut or * SO THAT: OBJECT MATCHES THE SIGNATURE OF THE METHOD

// aka - the following are the same:

// p1.distance(&p2)
// (&p1).distance(&p2)


fn test_squares() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    fn can_hold(&self, other_rec: &Rectangle) -> bool { // an immutable borrow!
        self.width > other_rec.width && self.height > other_rec.height
    }
}