fn main() {
    println!("Hello, world!");
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen { // On the other hand, with the method using trait objects, one Screen instance can hold a Vec that contains a Box<Button> as well as a Box<TextField>. 
    pub components: Vec<Box<dyn Draw>>, // NOTE: the dyn keyword - this is a trait object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // This works differently than defining a struct that uses a generic type parameter with trait bounds... A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
        }
    }
}

/**
 * E.G. THIS =>
 * 

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

======> Would restrict us to a Screen instance that has a list of components all of type Button or all of type TextField. If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.

 */