use speedy2d::dimen::Vector2;

pub type FnType = fn(f32, f32, f32) -> f32;
pub type FnType2 = fn(f32, f32, f32) -> Vector2<f32>;
pub type HeightMapType = Vec<Vec<f32>>;
