use crate::core::state::GameState;
use crate::story::dialogue::{print_dialogue, narrator};
use crate::data::characters::get_character;
use crate::data::enemies::get_enemy_team_chapter4;
use crate::battle::battle::Battle;
use crate::utils::input::wait_for_enter;

pub fn run_chapter4(state: &mut GameState) -> bool {
    narrator("CHAPTER 4: Collapse");
    print_dialogue("Elara", "Kita harus bersatu, Vex! Jangan biarkan keputusasaan menguasaimu.");
    print_dialogue("Vex", "Aku... tidak percaya kebaikan. Tapi mungkin kau benar.");
    state.add_affinity("Elara", "Vex", 20);
    let player_team = build_team(state);
    let enemy_team = get_enemy_team_chapter4();
    let mut battle = Battle::new(player_team, enemy_team);
    let win = battle.run();
    if win {
        narrator("The Null Sovereign akhirnya muncul di hadapanmu.");
        state.current_chapter = 5;
        wait_for_enter();
        true
    } else {
        false
    }
}

fn build_team(state: &GameState) -> Vec<crate::entity::character::Character> {
    state.active_team.iter().filter_map(|name| get_character(name)).collect()
}
