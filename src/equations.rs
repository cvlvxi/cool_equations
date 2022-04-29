use crate::game::{EqStrategy, GameScene};
use speedy2d::dimen::Vector2;

pub fn all_equation_scenes() -> Vec<GameScene> {
    vec![
        GameScene::GS2(EqStrategy::E1(e11)),
        GameScene::GS2(EqStrategy::E1(e12)),
        GameScene::GS2(EqStrategy::E1(e13)),
        GameScene::GS2(EqStrategy::E2(e21)),
    ]
}

pub fn e11(x: f32, y: f32, xt: f32) -> f32 {
    f32::cos((x / 20.0) + f32::sin(y+xt)) * 1000.0 + f32::sin((y+xt / 20.0) + f32::sin(xt+y)) * 1000.0
}

pub fn e12(x: f32, mut y: f32, mut xt: f32) -> f32 {
    let yt = xt;
    xt = xt / 2.0;
    y = y - (yt * 3.0);

    f32::cos((x / (20.0 / xt)) + f32::sin(y + xt)) * 1000.0
        + f32::sin((y + xt / (20.0 * xt)) + f32::sin(xt + y)) * 1000.0
}

pub fn e13(x: f32, y: f32, xt: f32) -> f32 {
    f32::tan(f32::cos((x / (20.0/xt)) + f32::sin(y+xt)) * 1000.0 + f32::sin((y+xt / (20.0*xt
    )) + f32::sin(xt+y)) * 1000.0)
}

pub fn e21(x: f32, mut y: f32, mut xt: f32) -> Vector2<f32> {
    Vector2 {
        x: f32::cos(x / (20.0 / xt) + f32::sin((y + xt) / 30.0)),
        y: f32::sin(x + xt / 10.0 + f32::cos(xt / 20.0)),
    }
}
