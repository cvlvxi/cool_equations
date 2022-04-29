use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D};

use crate::types::*;
use crate::utils::*;

#[derive(Debug)]
enum EqStrategy {
    E1(FnType),
    E2(FnType2)
}

#[derive(Debug)]
enum GameStrategy {
    GS1(HeightMapType),
    GS2(EqStrategy)
}

#[derive(Debug)]
pub struct Game {
    strategy: GameStrategy,
    width: u32,
    height: u32,
    x_translate: f32,
}

impl Game {
    pub fn new(size: usize, width: u32, height: u32) -> Self {
        let mut hm = vec![vec![0.0; size]; size];
        // let mut hm = Box::new([[0.0]; SIZE]);
        for x in 0..size {
            for y in 0..size {
                hm[x][y] = generate_height(x as f32, y as f32);
            }
        }
        // Self { strategy: GameStrategy::Type1(hm), width, height }
        // Self { strategy: GameStrategy::Type2(cool_equation2), width, height, x_translate: 0.0 }
        Self {
            strategy: GameStrategy::GS2(EqStrategy::E1(cool_equation)),
            width,
            height,
            x_translate: 0.0,
        }
    }
}

const max_height: f32 = 1000.0;
const num_bins: usize = 10;

impl WindowHandler for Game {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        match &self.strategy {
            GameStrategy::GS1(heightmap) => {
                let rect_distance = 5.0 as f32;
                for x in 0..heightmap.len() {
                    for y in 0..heightmap.len() {
                        // topleft bottomright
                        let r = {
                            let x = (x as f32 * rect_distance as f32);
                            let y = y as f32 * rect_distance as f32;
                            Rectangle::new(
                                Vector2 { x, y },
                                Vector2 {
                                    x: x + rect_distance,
                                    y: y + rect_distance,
                                },
                            )
                        };
                        graphics.draw_rectangle(
                            r,
                            height_to_col(
                                1.0,
                                0.0,
                                0.0,
                                read_height_map(&heightmap, x as f32, y as f32),
                                num_bins,
                                max_height,
                            ),
                        );
                    }
                }
            }
            GameStrategy::GS2(equation_strategy) => {
                let rect_distance: u32 = 5;
                for x in 0..self.width / rect_distance {
                    for y in 0..self.height / rect_distance {
                        let r = {
                            let x = x as f32 * rect_distance as f32;
                            let y = y as f32 * rect_distance as f32;
                            let rect_distance = rect_distance as f32;
                            Rectangle::new(
                                Vector2 { x, y },
                                Vector2 {
                                    x: x + rect_distance,
                                    y: y + rect_distance,
                                },
                            )
                        };
                        match equation_strategy {
                            EqStrategy::E1(equation) => {
                                graphics.draw_rectangle(
                                    r,
                                    height_to_col(
                                        0.0,
                                        1.0,
                                        0.0,
                                        equation(x as f32, y as f32, self.x_translate),
                                        num_bins,
                                        max_height,
                                    ),
                                );
                            },
                            EqStrategy::E2(equation) => {
                                let rect_distance = rect_distance as f32;
                                let x = x as f32;
                                let y = y as f32;
                                let result = equation(x, y, self.x_translate);
                                let xy = Vector2{x,y} * rect_distance;
                                graphics.draw_line(xy, xy+result, 1.0, Color::from_rgba(0.0, 1.0, 0.0, self.x_translate));
                            }
                        }
                    }
                }
                self.x_translate += 1.0;
            }
        }
        helper.request_redraw();
    }
}
