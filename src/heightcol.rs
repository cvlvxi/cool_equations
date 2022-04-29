use speedy2d::{color::Color, dimen::Vector2};

pub fn generate_height(x: f32, y: f32) -> f32 {
    ((((x / 10.0) + ((y + 40.0) / 20.0).sin()) * 6.0).cos() * 50.0
        + (((y / 4.0) + ((x / 100.0) * 10.0).sin()) * 50.0))
        / 8.0
}

pub fn read_height_map(height_map: &Vec<Vec<f32>>, x: f32, y: f32) -> f32 {
    // let x = 0f32.max((x.floor()).min(height_map.len() as f32-1.0));
    let x = x.clamp(0f32, height_map.len() as f32 - 1.0);
    let y = y.clamp(0f32, height_map.len() as f32 - 1.0);
    return height_map[x as usize][y as usize];
}

pub fn height_to_col(
    r: f32,
    g: f32,
    b: f32,
    height: f32,
    num_bins: usize,
    max_height: f32,
) -> Color {
    // let height = read_height_map(height_map, x as f32, y as f32);
    let alpha_interval_size = 1.0 / num_bins as f32;
    let interval_size = max_height / num_bins as f32;
    let num_intervals_for_height = (height / interval_size).floor();
    let alpha = num_intervals_for_height * alpha_interval_size;
    return Color::from_rgba(r, g, b, alpha as f32);
}
