use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator};
use crate::data::characters::get_character;
use crate::data::enemies::get_enemy_team_chapter1;
use crate::battle::battle::Battle;
use crate::utils::input::wait_for_enter;

pub fn run_chapter1(state: &mut GameState) -> bool {
    narrator("CHAPTER 1: Shattered Awakening");
    print_dialogue("Aiden", "Aetherfall hancur... Void Knight menghancurkan desa kita.");
    print_dialogue("Lyra", "Kita harus bertahan! Aku akan menyembuhkan luka kalian.");
    print_dialogue("Kael", "Aku akan melindungi kalian dengan perisai batu.");
    print_dialogue("Mira", "Angin membawa semangat baru! Ayo lawan mereka!");
    wait_for_enter();

    let player_team = get_initial_team(state);
    let enemy_team = get_enemy_team_chapter1();
    let mut battle = Battle::new(player_team, enemy_team);
    let win = battle.run();
    if win {
        narrator("Void Knight jatuh. Aiden dan kawan-kawan berhasil menyelamatkan desa.");
        print_dialogue("Mira", "Kita butuh sekutu lebih kuat... Aku dengar ada pahlawan cahaya di timur.");
        state.unlocked_characters.push("Elara".to_string());
        state.current_chapter = 2;
        wait_for_enter();
        true
    } else {
        false
    }
}

fn get_initial_team(state: &GameState) -> Vec<crate::entity::character::Character> {
    let mut team = vec![];
    for name in &state.active_team {
        if let Some(ch) = get_character(name) {
            team.push(ch);
        }
    }
    team
}
