use piston_window::{Context, G2d, Rectangle, Ellipse, ellipse, rectangle, line};

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
                Shape::Ellipse(fill_col, border_col, stroke_width, pos, size) => {
                    let mut elli = Ellipse::new(fill_col.to_arr());
                    if border_col.3 > 0 {
                        elli = elli.border(ellipse::Border{ color: border_col.to_arr(), radius: stroke_width });
                    }
                    elli.draw([pos.0, pos.1, size.0, size.1], &Default::default(), c.transform, g);
                },
                Shape::Rect(fill_col, border_col, stroke_width, pos, size) => {
                    let mut rect = Rectangle::new(fill_col.to_arr());
                    if border_col.3 > 0 {
                        rect = rect.border(rectangle::Border { color: border_col.to_arr(), radius: stroke_width });
                    }
                    rect.draw([pos.0, pos.1, size.0, size.1], &Default::default(), c.transform, g);
                },
                Shape::Line(col, stroke_width, from, to) => {
                    line(col.to_arr(), stroke_width, [from.0, from.1, to.0, to.1], c.transform, g);
                }
            }
        }
    }
}
