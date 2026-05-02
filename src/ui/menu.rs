use crate::utils::input::get_input;
use crate::ui::text::{title, show_affinity_panel, show_team_builder_prompt, show_synopsis};
use crate::core::state::GameState;
use crate::data::characters::get_character;
use crate::systems::team_builder::show_team_builder;
use crate::systems::equipment::show_equipment_menu;
use crate::utils::clear::clear_screen;

pub enum MenuChoice {
    NewGame,
    LoadGame,
    TeamBuilder,
    AffinityStatus,
    Equipment,
    Synopsis,
    Quit,
}

pub fn main_menu() -> MenuChoice {
    clear_screen();
    title();
    
    let border = "═".repeat(50);
    println!("{}┌{}┐{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    println!("{}│{:^50}│{}", 
        crate::core::config::COLOR_CYAN, "🎮 MAIN MENU 🎮", crate::core::config::COLOR_RESET);
    println!("{}├{}┤{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    println!("{}│  {}. ✨ Mulai Petualangan Baru{:<28}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_GREEN, "", crate::core::config::COLOR_RESET);
    println!("{}│  {}. 💾 Load Game{:<36}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_GREEN, "", crate::core::config::COLOR_RESET);
    println!("{}│  {}. 👥 Team Builder{:<34}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_GREEN, "", crate::core::config::COLOR_RESET);
    println!("{}│  {}. ❤️ Affinity Status{:<31}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_GREEN, "", crate::core::config::COLOR_RESET);
    println!("{}│  {}. ⚔️ Equipment{:<37}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_GREEN, "", crate::core::config::COLOR_RESET);
    println!("{}│  {}. 📖 Sinopsis Cerita{:<31}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_GREEN, "", crate::core::config::COLOR_RESET);
    println!("{}│  {}. 🚪 Keluar{:<40}│{}", 
        crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RED, "", crate::core::config::COLOR_RESET);
    println!("{}└{}┘{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    
    loop {
        let choice = get_input("Pilih [0-6]: ");
        match choice.as_str() {
            "1" => return MenuChoice::NewGame,
            "2" => return MenuChoice::LoadGame,
            "3" => {
                show_team_builder_prompt();
                return MenuChoice::TeamBuilder;
            }
            "4" => return MenuChoice::AffinityStatus,
            "5" => return MenuChoice::Equipment,
            "6" => return MenuChoice::Synopsis,
            "0" => return MenuChoice::Quit,
            _ => println!("{}❌ Pilihan salah. Masukkan 0-6.{}", 
                crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET),
        }
    }
}

pub fn show_synopsis_menu() {
    clear_screen();
    show_synopsis();
    println!("\n{}⏎ Tekan Enter untuk kembali ke menu...{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    get_input("");
    clear_screen();
}

pub fn show_affinity_menu(state: &GameState) {
    clear_screen();
    let mut characters = vec![];
    for name in &state.unlocked_characters {
        if let Some(ch) = get_character(name) {
            characters.push(ch);
        }
    }
    show_affinity_panel(state, &characters);
    println!("\n{}⏎ Tekan Enter untuk kembali ke menu...{}", 
        crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    get_input("");
    clear_screen();
}

pub fn handle_team_builder(state: &mut GameState) {
    clear_screen();
    show_team_builder(state);
    clear_screen();
}

pub fn handle_equipment(state: &mut GameState) {
    clear_screen();
    show_equipment_menu(state);
    clear_screen();
}
