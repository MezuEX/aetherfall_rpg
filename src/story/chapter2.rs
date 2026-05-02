use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator};
use crate::data::characters::get_character;
use crate::data::enemies::get_enemy_team_chapter2;
use crate::battle::battle::Battle;
use crate::utils::input::wait_for_enter;

pub fn run_chapter2(state: &mut GameState) -> bool {
    narrator("CHAPTER 2: Echoes of Light");
    print_dialogue("Elara", "Aku Elara, Knight of Dawn. Akan kubantu melawan kegelapan.");
    print_dialogue("Aiden", "Terima kasih. Tapi musuh makin kuat.");
    let player_team = build_team(state);
    let enemy_team = get_enemy_team_chapter2();
    let mut battle = Battle::new(player_team, enemy_team);
    let win = battle.run();
    if win {
        narrator("Setelah pertempuran, Vex muncul dari bayangan.");
        print_dialogue("Vex", "Kalian lemah. Tapi... mungkin aku bisa membantu.");
        state.unlocked_characters.push("Vex".to_string());
        state.add_affinity("Elara", "Vex", 10);
        state.current_chapter = 3;
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
    if state.unlocked_characters.contains(&"Elara".to_string()) && !state.active_team.contains(&"Elara".to_string()) {
        if let Some(elara) = get_character("Elara") {
            team.push(elara);
        }
    }
    team
}
