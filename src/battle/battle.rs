use crate::entity::character::Character;
use crate::battle::turn::TurnManager;
use crate::systems::synergy_system::apply_synergy;
use crate::ui::battle_ui::{display_team_status, battle_start_header, battle_result};
use crate::core::state::GameState;
use crate::systems::equipment::random_equipment_drop;
use crate::utils::clear::wait_and_clear;
use std::cell::RefCell;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleSaveData {
    pub turn: u32,
    pub player_team: Vec<Character>,
    pub enemy_team: Vec<Character>,
    pub current_character_idx: usize,
}

thread_local! {
    static BATTLE_STATE: RefCell<Option<Rc<RefCell<GameState>>>> = RefCell::new(None);
}

pub fn set_battle_state(state: Rc<RefCell<GameState>>) {
    BATTLE_STATE.with(|s| {
        *s.borrow_mut() = Some(state);
    });
}

pub fn get_battle_state() -> Option<Rc<RefCell<GameState>>> {
    BATTLE_STATE.with(|s| {
        s.borrow().clone()
    })
}

pub fn clear_battle_state() {
    BATTLE_STATE.with(|s| {
        *s.borrow_mut() = None;
    });
}

pub struct Battle {
    pub player_team: Vec<Character>,
    pub enemy_team: Vec<Character>,
    pub game_state: Option<Rc<RefCell<GameState>>>,
    pub current_turn: u32,
}

impl Battle {
    pub fn new(player_team: Vec<Character>, enemy_team: Vec<Character>) -> Self {
        Battle { 
            player_team, 
            enemy_team,
            game_state: None,
            current_turn: 1,
        }
    }

    pub fn with_state(mut self, state: Rc<RefCell<GameState>>) -> Self {
        self.game_state = Some(state.clone());
        set_battle_state(state);
        self
    }

    pub fn run(&mut self) -> bool {
        apply_synergy(&mut self.player_team);
        
        if let Some(ref state_rc) = self.game_state {
            let state = state_rc.borrow();
            Self::apply_affinity_bonus(&mut self.player_team, &state);
        }
        
        battle_start_header();
        display_team_status(&self.player_team, "TIM PEMAIN", crate::core::config::COLOR_CYAN);
        display_team_status(&self.enemy_team, "MUSUH", crate::core::config::COLOR_RED);
        wait_and_clear();
        
        while Self::is_team_alive(&self.player_team) && Self::is_team_alive(&self.enemy_team) {
            let battle_ended = TurnManager::run_turn(&mut self.player_team, &mut self.enemy_team, self.current_turn);
            
            if battle_ended {
                if Self::is_team_alive(&self.player_team) && !Self::is_team_alive(&self.enemy_team) {
                    battle_result(true);
                    
                    let mut rng = rand::thread_rng();
                    if let Some(equip) = random_equipment_drop(&mut rng) {
                        println!("\n{}🎁 Kamu mendapatkan equipment: {} (+{} ATK, +{} DEF){}", 
                            crate::core::config::COLOR_GREEN, 
                            equip.name, 
                            equip.atk_bonus, 
                            equip.def_bonus, 
                            crate::core::config::COLOR_RESET);
                        if let Some(ref mut state_rc) = self.game_state {
                            let mut state = state_rc.borrow_mut();
                            if let Some(first_alive) = self.player_team.iter_mut().find(|c| c.is_alive()) {
                                state.equip_item(&first_alive.name, equip);
                            }
                        }
                    }
                    
                    wait_and_clear();
                    clear_battle_state();
                    return true;
                } else {
                    battle_result(false);
                    wait_and_clear();
                    clear_battle_state();
                    return false;
                }
            }
            self.current_turn += 1;
        }
        
        if Self::is_team_alive(&self.player_team) && !Self::is_team_alive(&self.enemy_team) {
            battle_result(true);
            wait_and_clear();
            clear_battle_state();
            true
        } else {
            battle_result(false);
            wait_and_clear();
            clear_battle_state();
            false
        }
    }
    
    fn apply_affinity_bonus(team: &mut [Character], state: &GameState) {
        for i in 0..team.len() {
            for j in i+1..team.len() {
                let affinity = state.get_affinity(&team[i].name, &team[j].name);
                if affinity >= 30 {
                    team[i].atk += 5;
                    team[j].atk += 5;
                    println!("{}❤️ Bonus affinity! ATK {} dan {} +5 karena hubungan dekat!{}", 
                        crate::core::config::COLOR_MAGENTA, 
                        team[i].name, 
                        team[j].name, 
                        crate::core::config::COLOR_RESET);
                }
            }
        }
    }

    fn is_team_alive(team: &[Character]) -> bool {
        team.iter().any(|c| c.is_alive())
    }
}
