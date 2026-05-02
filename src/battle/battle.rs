use crate::entity::character::Character;
use crate::battle::turn::TurnManager;
use crate::systems::synergy_system::apply_synergy;
use crate::ui::battle_ui::{display_team_status, battle_header, battle_result};
use crate::core::state::GameState;

pub struct Battle {
    pub player_team: Vec<Character>,
    pub enemy_team: Vec<Character>,
    pub game_state: Option<GameState>,
}

impl Battle {
    pub fn new(player_team: Vec<Character>, enemy_team: Vec<Character>) -> Self {
        Battle { 
            player_team, 
            enemy_team,
            game_state: None,
        }
    }

    pub fn run(&mut self) -> bool {
        apply_synergy(&mut self.player_team);
        
        if let Some(state) = &self.game_state {
            Self::apply_affinity_bonus(&mut self.player_team, state);
        }
        
        println!("\n{}═{} BATTLE START {}═{}{}", 
            crate::core::config::COLOR_YELLOW,
            "═".repeat(15), 
            "═".repeat(15), 
            "═".repeat(15),
            crate::core::config::COLOR_RESET);
        
        let mut turn = 1;
        while Self::is_team_alive(&self.player_team) && Self::is_team_alive(&self.enemy_team) {
            battle_header(turn);
            display_team_status(&self.player_team, "TIM PEMAIN", crate::core::config::COLOR_CYAN);
            display_team_status(&self.enemy_team, "MUSUH", crate::core::config::COLOR_RED);
            
            let win = TurnManager::run_turn(&mut self.player_team, &mut self.enemy_team);
            if win {
                battle_result(true);
                return true;
            }
            turn += 1;
        }
        battle_result(false);
        false
    }
    
    fn apply_affinity_bonus(team: &mut [Character], state: &GameState) {
        for i in 0..team.len() {
            for j in i+1..team.len() {
                let affinity = state.get_affinity(&team[i].name, &team[j].name);
                if affinity >= 30 {
                    team[i].atk += 5;
                    team[j].atk += 5;
                    println!("{}❤️ Bonus affinity! ATK {} dan {} +5 karena hubungan dekat!{}", 
                        crate::core::config::COLOR_MAGENTA, team[i].name, team[j].name, crate::core::config::COLOR_RESET);
                }
            }
        }
    }

    fn is_team_alive(team: &[Character]) -> bool {
        team.iter().any(|c| c.is_alive())
    }
}
