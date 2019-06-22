fn main() {
    println!("Hello, world!");
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // NOTE: the dyn keyword - this is a trait object
}