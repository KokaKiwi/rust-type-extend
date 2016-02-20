use std::fmt;

extend! {
    pub impl<T: fmt::Debug> VecExt<T> for Vec<T> {
        fn print(&self) {
            println!("{:?}", self);
        }
    }
}
