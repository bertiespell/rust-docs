fn main() {
    another_function(5);
    let x = five();
    println!("The value of x is {}", x);
    let z = plus_one(x);
    println!("The value of z is {}", z);
}

fn another_function(x: i32) {
    println!("Hello {}", x);

    let y = {
        let x = 3;
        x + 1 // adding a ; here would turn it into a statement instead of an expression. Expression endings don't have a ;
    };

    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}