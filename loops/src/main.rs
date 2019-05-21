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

    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("Element is {}", element);
    };

    for number in (1..4).rev() {
        println!("Number is {}", number);
    };
}
