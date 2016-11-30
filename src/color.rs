#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn to_arr(self) -> [f32; 4] {
        [ self.0 as f32 / 255.
        , self.1 as f32 / 255.
        , self.2 as f32 / 255.
        , self.3 as f32 / 255. ]
    }
}
