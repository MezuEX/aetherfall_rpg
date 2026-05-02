use crate::core::state::GameState;
use crate::core::config::*;
use crate::story::chapter1::run_chapter1;
use crate::story::chapter2::run_chapter2;
use crate::story::chapter3::run_chapter3;
use crate::story::chapter4::run_chapter4;
use crate::story::final_battle::run_final_battle;
use crate::story::ending::show_ending;
use crate::ui::menu::{main_menu, MenuChoice, show_affinity_menu};
use crate::utils::input::wait_for_enter;
use std::fs;

pub fn run_game() {
    println!("\x1b[35m{} v{}\x1b[0m", GAME_NAME, VERSION);
    loop {
        match main_menu() {
            MenuChoice::NewGame => start_new_game(),
            MenuChoice::LoadGame => load_game(),
            MenuChoice::TeamBuilder => {
                println!("Fitur team builder akan segera hadir!");
                wait_for_enter();
                continue;
            }
            MenuChoice::AffinityStatus => {
                if let Some(state) = try_load_game_state() {
                    show_affinity_menu(&state);
                } else {
                    println!("Belum ada save game. Mulai petualangan baru dulu!");
                    wait_for_enter();
                }
                continue;
            }
            MenuChoice::Quit => break,
        }
    }
}

fn try_load_game_state() -> Option<GameState> {
    if let Ok(data) = fs::read_to_string(SAVE_FILE) {
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

fn start_new_game() {
    let mut state = GameState::new();
    println!("\nMemulai petualangan baru...");
    wait_for_enter();

    let win = run_chapter1(&mut state);
    if !win { return; }
    save_game(&state);
    
    let win = run_chapter2(&mut state);
    if !win { return; }
    save_game(&state);
    
    let win = run_chapter3(&mut state);
    if !win { return; }
    save_game(&state);
    
    let win = run_chapter4(&mut state);
    if !win { return; }
    save_game(&state);
    
    let ending_id = run_final_battle(&mut state);
    show_ending(&ending_id);
    println!("\nTerima kasih telah bermain!");
}

fn load_game() {
    if let Ok(data) = fs::read_to_string(SAVE_FILE) {
        if let Ok(mut state) = serde_json::from_str::<GameState>(&data) {
            println!("Load berhasil. Melanjutkan chapter {}", state.current_chapter);
            wait_for_enter();
            
            match state.current_chapter {
                1 => { run_chapter1(&mut state); }
                2 => { run_chapter2(&mut state); }
                3 => { run_chapter3(&mut state); }
                4 => { run_chapter4(&mut state); }
                5 => { 
                    let ending_id = run_final_battle(&mut state);
                    show_ending(&ending_id);
                }
                _ => println!("Chapter tidak dikenal."),
            }
            return;
        }
    }
    println!("Tidak ada save file.");
}

pub fn save_game(state: &GameState) {
    if let Ok(json) = serde_json::to_string(state) {
        let _ = fs::write(SAVE_FILE, json);
        println!("\x1b[32m💾 Game tersimpan pada chapter {}\x1b[0m", state.current_chapter);
    }
}
