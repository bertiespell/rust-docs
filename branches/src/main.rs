fn main() {
    let number = 3;

    if number > 5 {
        println!("Condition was true");
    } else if number == 3 {
        println!("Number was 3");
    } else {
        println!("Condition was false")
    }

    let condition = false;
    let x = if condition { 
        5
    } else { 
        6
    };

    println!("X was equal to {}", x);
}

