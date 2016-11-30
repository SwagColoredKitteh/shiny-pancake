use vec2::*;
use color::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Shape {
    Ellipse(Color, Vec2, Vec2),
    Rect(Color, Vec2, Vec2),
    Line(Color, Vec2, Vec2)
}
