use speedy2d::dimen::Vector2;

pub fn cool_equation(x: f32, mut y: f32, mut xt: f32) -> f32 {
    let yt = xt;
    xt = xt / 2.0;
    y = y - (yt * 3.0);

    f32::cos((x / (20.0 / xt)) + f32::sin(y + xt)) * 1000.0
        + f32::sin((y + xt / (20.0 * xt)) + f32::sin(xt + y)) * 1000.0
}

pub fn cool_equation2(x: f32, mut y: f32, mut xt: f32) -> Vector2<f32> {
    Vector2 {
        x: f32::cos(x / (20.0 / xt) + f32::sin((y + xt) / 30.0)),
        y: f32::sin(x + xt / 10.0 + f32::cos(xt / 20.0)),
    }
}