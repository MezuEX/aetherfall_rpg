use crate::utils::input::get_input;
use crate::ui::text::{title, show_team_builder_prompt, show_affinity_panel};
use crate::core::state::GameState;
use crate::data::characters::get_character;

pub enum MenuChoice {
    NewGame,
    LoadGame,
    TeamBuilder,
    AffinityStatus,
    Quit,
}

pub fn main_menu() -> MenuChoice {
    title();
    println!("┌──────────────────────────────────────┐");
    println!("│ 1. ✨ Mulai Petualangan Baru        │");
    println!("│ 2. 💾 Load Game                     │");
    println!("│ 3. 👥 Team Builder                  │");
    println!("│ 4. ❤️  Affinity Status              │");
    println!("│ 5. 🚪 Keluar                        │");
    println!("└──────────────────────────────────────┘");
    
    loop {
        let choice = get_input("Pilih [1-5]: ");
        match choice.as_str() {
            "1" => return MenuChoice::NewGame,
            "2" => return MenuChoice::LoadGame,
            "3" => {
                show_team_builder_prompt();
                return MenuChoice::TeamBuilder;
            }
            "4" => return MenuChoice::AffinityStatus,
            "5" => return MenuChoice::Quit,
            _ => println!("{}❌ Pilihan salah. Masukkan 1-5.{}", 
                crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET),
        }
    }
}

pub fn show_affinity_menu(state: &GameState) {
    let mut characters = vec![];
    for name in &state.unlocked_characters {
        if let Some(ch) = get_character(name) {
            characters.push(ch);
        }
    }
    show_affinity_panel(state, &characters);
    println!("\nTekan Enter untuk kembali ke menu...");
    get_input("");
}
