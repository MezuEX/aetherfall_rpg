use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator};
use crate::data::characters::get_character;
use crate::data::enemies::get_final_boss;
use crate::battle::battle::Battle;
use crate::utils::input::get_input;

pub fn run_final_battle(state: &mut GameState) -> String {
    narrator("FINAL BATTLE: The Null Sovereign");
    print_dialogue("Null Sovereign", "Keseimbangan adalah ilusi. Akan ku hapus semua elemen.");
    let player_team = build_team(state);
    let boss_team = vec![get_final_boss()];
    let mut battle = Battle::new(player_team, boss_team);
    let win = battle.run();
    if win {
        narrator("Sovereign jatuh. Sekarang, pilihan ada di tanganmu.");
        println!("\nApa yang akan kau lakukan?");
        println!("1. Satukan semua elemen → Dunia stabil namun rapuh.");
        println!("2. Hapus elemen → Dunia damai, hampa.");
        println!("3. Biarkan chaos → Dunia liar, penuh petualangan.");
        let choice = get_input("Pilih 1/2/3: ");
        match choice.as_str() {
            "1" => "harmony",
            "2" => "void",
            "3" => "chaos",
            _ => "default",
        }.to_string()
    } else {
        "default".to_string()
    }
}

fn build_team(state: &GameState) -> Vec<crate::entity::character::Character> {
    state.active_team.iter().filter_map(|name| get_character(name)).collect()
}
