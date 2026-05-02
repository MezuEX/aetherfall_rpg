use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator};
use crate::data::characters::get_character;
use crate::data::enemies::get_enemy_team_chapter3;
use crate::battle::battle::Battle;
use crate::utils::input::wait_for_enter;

pub fn run_chapter3(state: &mut GameState) -> bool {
    narrator("CHAPTER 3: Fractured Truth");
    print_dialogue("Orion", "Aku Orion, pengendali waktu. Kalian butuh bantuanku.");
    print_dialogue("Aiden", "Semakin banyak sekutu, semakin baik. Tapi apa tujuan sebenarnya?");
    print_dialogue("Nyx", "Hahaha, kegelapan adalah kebebasan. Ayo kita hancurkan dunia lama!");
    state.unlocked_characters.push("Orion".to_string());
    state.unlocked_characters.push("Nyx".to_string());
    let player_team = build_team(state);
    let enemy_team = get_enemy_team_chapter3();
    let mut battle = Battle::new(player_team, enemy_team);
    let win = battle.run();
    if win {
        narrator("Kamu mulai menyadari bahwa dirimu bukan manusia biasa...");
        print_dialogue("Weaver", "Aku adalah wadah Core Element.");
        state.global_flags.insert("weaver_awaken".to_string(), true);
        state.current_chapter = 4;
        wait_for_enter();
        true
    } else {
        false
    }
}

fn build_team(state: &GameState) -> Vec<crate::entity::character::Character> {
    let mut team = vec![];
    for name in &state.active_team {
        if let Some(ch) = get_character(name) {
            team.push(ch);
        }
    }
    for name in &["Orion", "Nyx"] {
        if state.unlocked_characters.contains(&name.to_string()) && !state.active_team.contains(&name.to_string()) {
            if let Some(ch) = get_character(name) {
                team.push(ch);
            }
        }
    }
    team
}
