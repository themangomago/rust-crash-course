use piston_window::types::Color;

use crate::snake::Snake;

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.8, 0.0, 0.0, 0.5];

const MOVING_TIME: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;


pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

https://www.youtube.com/watch?v=DnT_7M7L7vo&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&index=6