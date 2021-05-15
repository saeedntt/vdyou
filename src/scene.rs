use crate::frame::Frame;
use crate::mshape::MShape;
use std::ops::Range;
use draw::Shape;

pub struct Scene {
    frames: Vec<Frame>
}

impl Scene {
    fn addAnimation(&mut self, shapes: Vec<MShape>, frames:Range<u64>)-> Result<(), SceneError> {
        if shapes.len() != (frames.end - frames.start) as usize {
            Err(SceneError::new())
        }

        if frames.end >= self.frames.len() as u64 {
            let diff = frames.end - self.frames.len();
            self.frames.append(&mut vec![Frame::new(); diff]);
        }

        self.frames.iter_mut()
            .skip((frames.start - 1) as usize)
            .zip(shapes)
            .map(|(f, s)| {
                f.add_shape(s);
            });

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct SceneError; //TODO: Add useful info about the error

impl SceneError {
    fn new() -> Self {
        SceneError{}
    }
}