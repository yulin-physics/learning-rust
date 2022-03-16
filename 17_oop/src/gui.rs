pub trait Draw {
    fn draw(&self);
}
/////////////////trait objects////////////////////////
// dynamic dispatch

// trait objects must use a pointer
//  A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

////////////////generic type and trait bound///////////
// static dispatch

//pub struct Screen<T: Draw> {
// pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

