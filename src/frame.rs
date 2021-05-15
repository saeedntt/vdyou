use draw::Style;

use crate::mshape::MShape;

pub struct Frame {
    shapes: Vec<MShape>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: MShape) {
        self.shapes.push(shape);
    }
}

