use piston_window::{Context, G2d, rectangle, ellipse, line};

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

    pub fn render(&self, c: Context, g: &mut G2d) {
        for shape in self.shapes.iter() {
            match *shape {
                Shape::Ellipse(col, pos, size) => {
                    ellipse(col.to_arr(), [pos.0, pos.1, size.0, size.1], c.transform, g);
                },
                Shape::Rect(col, pos, size) => {
                    rectangle(col.to_arr(), [pos.0, pos.1, size.0, size.1], c.transform, g);
                },
                Shape::Line(col, from, to) => {
                    line(col.to_arr(), 2., [from.0, from.1, to.0, to.1], c.transform, g);
                }
            }
        }
    }
}
