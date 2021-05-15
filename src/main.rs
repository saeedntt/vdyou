mod frame;
mod transformation;
mod scene;
mod mshape;

use draw::{Canvas, Drawing, Shape, Point, Style, Color, SvgRenderer, render, RGB};
use draw::render::bitmap::PNGRenderer;
use draw::render::Renderer;
use std::cmp::{max, min};
use std::ops::Sub;

fn main() {
    let mut canvas = Canvas::new(3840, 2160);
    let background = Drawing::new()
        .with_shape(Shape::Rectangle {
            height : 2180,
            width: 3860
        })
        .with_style(Style::filled(Color::random()))
        .with_xy(-10.0, -10.0);

    canvas.display_list.add(background);

    let rect = Drawing::new()
        .with_shape(Shape::Rectangle {
            height: 250,
            width: 250,
        })
        .with_xy(50.0, 50.0)
        .with_style(Style::filled(Color::random()));

    canvas.display_list.add(rect);

    render::save(&canvas, "/home/rust/output/out.svg", SvgRenderer::new())
        .expect("Output folder cannot be created");

}

