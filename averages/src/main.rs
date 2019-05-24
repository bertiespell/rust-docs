fn main() {
    let numbers = vec!(1,2,3,4,5);
    let average = average(numbers);
    println!("Average : {} ", average);
}

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn average(numbers: Vec<i32>) -> usize {
    let mut total = 0;
    for number in numbers.iter() {
        total += number;
    }
    return total as usize/numbers.len() // is there a way to do this without typecasting as
}