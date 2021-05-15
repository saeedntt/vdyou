use crate::scene::Scene;
use crate::mshape::MShape;

pub struct Director {
    scene: Scene,
    shapes: Vec<MShape>
}

impl Director {
    fn new(scene: Scene) -> Self {
        Director {
            scene: scene,
            shapes: vec![]
        }
    }

    fn add_shape(&mut self, shape: MShape) {
        self.shapes.append(MShape);
    }

    // fn play_scenario(); //damn that's too deep :))
}