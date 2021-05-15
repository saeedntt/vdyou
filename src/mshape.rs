use draw::Drawing;

#[derive(Clone)]
pub struct MShape {
    pub inner: Drawing
}

impl MShape {
    pub fn new(shape: Drawing) -> Self {
        MShape {
            inner: shape
        }
    }

    pub fn move_x(self, to_x: f32, in_shapes: u64) -> Vec<Drawing> {
        let p = self.inner.position;
        let moves = (to_x - p.x) / in_shapes;
        let mut shapes: Vec<Drawing> = Vec::new();
        for m in (0..in_shapes) {
            shapes.push(self.inner.with_xy(p.x + m * moves, p.y))
        }
        shapes
    }
}

pub trait Mshaper {
    fn to_mshape(&self) -> MShape;
}

impl Mshaper for Drawing {
    fn to_mshape(self) -> MShape {
        MShape::new(self)
    }
}