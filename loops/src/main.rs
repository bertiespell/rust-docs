fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("Result was {}", result);

    let mut number = 3;

    while number != 0 {
        number -= 1;
    };

    println!("Lift off!");
}
