fn main() {
    let numbers = vec!(1, 2, 3, 4, 5);
    let letters = vec!('a', 'b', 'c', 'd');
    let largest_number = largest_i32(&numbers);
    let largest_letter = largest_char(&letters);
    println!("Largest stuff {}, {}", largest_number, largest_letter);
    example_mixup();
}

// place generics in the function signature

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &letter in list.iter() {
        if letter > largest {
            largest = letter;
        }
    }
    largest
}


// both of these functions are the same! Other than the type - let's use generics!

// If we don’t want to restrict the largest function to the types that implement the Copy trait, we could specify that T has the trait bound Clone instead of Copy. Then we could clone each value in the slice when we want the largest function to have ownership. Using the clone function means we’re potentially making more heap allocations in the case of types that own heap data like String, and heap allocations can be slow if we’re working with large amounts of data.

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // this needs to implement the copy trait, otherwise we're on a non-copy slice and can't move out
    for &item in list.iter() {
        // But we get an error: binary operation `>` cannot be applied to type `T`
        if item > largest {
            // In order to compare, the list has to be of a type that has the trait std::cmp::PartialOrd - which is a TRAIT
            // can only use values of a type that can be ordered.

            // TBD: Define a generic type that has a particular trait
            largest = item;
        }
    }
    largest
}

// we can also use generic traits in struct definitions

struct Point<T> {
    x: T,
    y: T
}

fn example() {
    let little_point: Point<i32> = Point {
        x: 5,
        y: 6,
    };

    // actually, it can be interred

    let inferred_type_point = Point { x: 10, y: 9 };
    let float_type_point = Point { x: 12.2, y: 45.0 };
    let float_and_int_point = TwoTypePoint { x: 10, y: 22.2};
}

// If you need two types
struct TwoTypePoint<T, U> {
    x: T,
    y: U
}

// can also use generics in enums

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, U> {
    Ok(T),
    Err(U),
}

// in method definitions

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl <T, U> TwoTypePoint<T, U> {
    fn mixup<V, W>(self, other: TwoTypePoint<V, W>) -> TwoTypePoint<T, W> { // why does this take the whole reference
        TwoTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn example_mixup() {
    let point1 = TwoTypePoint {
        x: 1,
        y: 10.4,
    };
    let point2 = TwoTypePoint {
        x: "hello", // string slice
        y: 'n', // char
    };
    let p3 = point1.mixup(point2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Good news: code with generics doesn't run any slower
// It uses Monomorphization - i.e. it fills in concrete types at compile time
// basically the reverse flow of refactoring the two generic types
// The compiler recreates out the generic types