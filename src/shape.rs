use vec2::*;
use color::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Shape {
    Ellipse(Color, Color, f64, Vec2, Vec2),
    Rect(Color, Color, f64, Vec2, Vec2),
    Line(Color, f64, Vec2, Vec2)
}
