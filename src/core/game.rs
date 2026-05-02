use crate::core::state::GameState;
use crate::core::config::*;
use crate::story::chapter1::run_chapter1;
use crate::story::chapter2::run_chapter2;
use crate::story::chapter3::run_chapter3;
use crate::story::chapter4::run_chapter4;
use crate::story::final_battle::run_final_battle;
use crate::story::ending::show_ending;
use crate::ui::menu::{main_menu, MenuChoice, show_affinity_menu, handle_team_builder, handle_equipment, show_synopsis_menu};
use crate::utils::input::wait_for_enter;
use crate::utils::clear::{clear_screen, wait_and_clear};
use crate::systems::relationship_events::check_all_relationship_events;
use crate::core::save_manager::{save_game, load_game, choose_load_slot, choose_save_slot};

pub fn run_game() {
    clear_screen();
    println!("\x1b[35m{} v{}\x1b[0m", GAME_NAME, VERSION);
    loop {
        match main_menu() {
            MenuChoice::NewGame => start_new_game(),
            MenuChoice::LoadGame => load_game_menu(),
            MenuChoice::TeamBuilder => {
                if let Some(mut state) = try_load_last_save() {
                    handle_team_builder(&mut state);
                    choose_and_save(&state);
                } else {
                    println!("Belum ada save game. Mulai petualangan baru dulu!");
                    wait_for_enter();
                    clear_screen();
                }
                continue;
            }
            MenuChoice::AffinityStatus => {
                if let Some(state) = try_load_last_save() {
                    show_affinity_menu(&state);
                } else {
                    println!("Belum ada save game. Mulai petualangan baru dulu!");
                    wait_for_enter();
                    clear_screen();
                }
                continue;
            }
            MenuChoice::Equipment => {
                if let Some(mut state) = try_load_last_save() {
                    handle_equipment(&mut state);
                    choose_and_save(&state);
                } else {
                    println!("Belum ada save game. Mulai petualangan baru dulu!");
                    wait_for_enter();
                    clear_screen();
                }
                continue;
            }
            MenuChoice::Synopsis => {
                show_synopsis_menu();
                continue;
            }
            MenuChoice::Quit => break,
        }
    }
}

fn try_load_last_save() -> Option<GameState> {
    for slot in (1..=MAX_SAVE_SLOTS).rev() {
        if let Some(state) = load_game(slot) {
            return Some(state);
        }
    }
    None
}

fn load_game_menu() {
    if let Some(slot) = choose_load_slot() {
        if let Some(mut state) = load_game(slot) {
            clear_screen();
            println!("Load berhasil. Melanjutkan chapter {}", state.current_chapter);
            wait_and_clear();
            
            let result = match state.current_chapter {
                1 => run_chapter1(&mut state),
                2 => run_chapter2(&mut state),
                3 => run_chapter3(&mut state),
                4 => run_chapter4(&mut state),
                5 => { 
                    check_all_relationship_events(&mut state);
                    let ending_id = run_final_battle(&mut state);
                    show_ending(&ending_id);
                    true
                }
                _ => {
                    println!("Chapter tidak dikenal.");
                    false
                }
            };
            
            if result {
                save_game(&state, slot);
            }
        }
    } else {
        wait_and_clear();
    }
}

fn start_new_game() {
    let mut state = GameState::new();
    clear_screen();
    println!("\nMemulai petualangan baru...");
    wait_and_clear();

    if !run_chapter1(&mut state) { return; }
    choose_and_save(&state);
    
    if !run_chapter2(&mut state) { return; }
    choose_and_save(&state);
    
    if !run_chapter3(&mut state) { return; }
    choose_and_save(&state);
    
    if !run_chapter4(&mut state) { return; }
    choose_and_save(&state);
    
    check_all_relationship_events(&mut state);
    choose_and_save(&state);
    
    let ending_id = run_final_battle(&mut state);
    show_ending(&ending_id);
    println!("\nTerima kasih telah bermain!");
    wait_and_clear();
}

fn choose_and_save(state: &GameState) {
    if let Some(slot) = choose_save_slot() {
        save_game(state, slot);
    }
    wait_and_clear();
}

pub fn try_save_game(state: &GameState) {
    if let Some(slot) = choose_save_slot() {
        save_game(state, slot);
    }
}
