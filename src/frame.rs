use shape::*;

#[derive(Clone, Debug)]
pub struct Frame {
    pub shapes: Vec<Shape>
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            shapes: Vec::new()
        }
    }
}
