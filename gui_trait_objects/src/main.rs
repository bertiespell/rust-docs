fn main() {
    println!("Hello, world!");
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // NOTE: the dyn keyword - this is a trait object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // This works differently than defining a struct that uses a generic type parameter with trait bounds... A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
        }
    }
}