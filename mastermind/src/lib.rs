use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple
}

const COLORS: [Color; 6] = [
    Color::Red,
    Color::Orange,
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Purple
];

fn generate_code<'a>() -> Vec<&'a Color> {
    let mut rng = thread_rng();
    // start with an empty array - length 4
    // .map(|c| *c)
    (0..4).into_iter().map(|_| rng.gen_range(0, 6)).map(|i| &COLORS[i]).collect()
    // map over it - generate random number (0-5)
    // grab color at that index
    // populate it!
}   

pub fn add_one_to_many(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|n| n + 1).collect::<Vec<i32>>()
    // return nums.map(add_one)
}

pub fn add_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_adds_one() {
        assert_eq!(add_one(2), 3);
    }

    #[test]
    fn it_adds_one_to_many(){
        assert_eq!(add_one_to_many(vec![1,2,3]), vec![2,3,4]);
    }

    #[test]
    fn it_populates_array_of_colors() {
        assert_eq!(generate_code(), vec![&Color::Red]);
    }
}
