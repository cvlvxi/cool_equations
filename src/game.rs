use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

use crate::equations::*;
use crate::heightmap::*;
use crate::types::*;

#[derive(Debug)]
enum EqStrategy {
    E1(FnType),
    E2(FnType2),
}

#[derive(Debug)]
enum GameScene {
    GS1(HeightMapType),
    GS2(EqStrategy),
}

#[derive(Debug)]
pub struct Game {
    scenes: Vec<GameScene>,
    width: u32,
    height: u32,
    delta: f32,
    curr_scene: usize,
}

impl Game {
    pub fn new(size: usize, width: u32, height: u32) -> Self {
        let mut hm = vec![vec![0.0; size]; size];
        for x in 0..size {
            for y in 0..size {
                hm[x][y] = generate_height(x as f32, y as f32);
            }
        }

        Self {
            scenes: vec![
                GameScene::GS2(EqStrategy::E1(cool_equation)),
                GameScene::GS2(EqStrategy::E2(cool_equation2)),
                GameScene::GS1(hm),
            ],
            width,
            height,
            delta: 0.0,
            curr_scene: 0,
        }
    }
}

const max_height: f32 = 1000.0;
const num_bins: usize = 10;

impl WindowHandler for Game {
    fn on_key_down(
        &mut self,
        _: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        match virtual_key_code {
            Some(key_code) => match key_code {
                VirtualKeyCode::Right => {
                    if self.curr_scene + 1 >= self.scenes.len() {
                        self.curr_scene = 0;
                    } else {
                        self.curr_scene = self.curr_scene + 1;
                    }
                }
                VirtualKeyCode::Left => {
                    if self.curr_scene == 0 {
                        self.curr_scene = self.scenes.len() - 1
                    } else {
                        self.curr_scene = self.curr_scene - 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        // println!("{:?}: virtual_key_code. {:?}: scancode", virtual_key_code, scancode);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        let scene = &self.scenes[self.curr_scene];
        match scene {
            GameScene::GS1(heightmap) => {
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
            GameScene::GS2(equation_strategy) => {
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
                                        equation(x as f32, y as f32, self.delta),
                                        num_bins,
                                        max_height,
                                    ),
                                );
                            }
                            EqStrategy::E2(equation) => {
                                let rect_distance = rect_distance as f32;
                                let x = x as f32;
                                let y = y as f32;
                                let result = equation(x, y, self.delta);
                                let xy = Vector2 { x, y } * rect_distance;
                                graphics.draw_line(
                                    xy,
                                    xy + result,
                                    1.0,
                                    Color::from_rgba(0.0, 1.0, 0.0, self.delta),
                                );
                            }
                        }
                    }
                }
                self.delta += 1.0;
            }
        }
        helper.request_redraw();
    }
}
