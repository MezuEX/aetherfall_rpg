mod core;
mod battle;
mod entity;
mod systems;
mod story;
mod data;
mod ui;
mod utils;

use crate::core::game::run_game;

fn main() {
    run_game();
}
