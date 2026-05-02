use crate::entity::character::Character;
use crate::entity::effect::Effect;
use crate::battle::ai::EnemyAI;
use crate::core::config::{ENERGY_PER_TURN, ULTIMATE_ENERGY_COST};
use crate::utils::input::{get_input, get_usize, wait_for_enter_simple};
use crate::ui::battle_ui::{display_hp_bar, display_team_status, battle_header};
use crate::utils::clear::clear_screen;
use crate::core::game::try_save_game;
use crate::battle::battle::get_battle_state;
use crate::battle::battle::BattleSaveData;

pub struct TurnManager;

impl TurnManager {
    pub fn run_turn(player_team: &mut Vec<Character>, enemy_team: &mut Vec<Character>, turn: u32) -> bool {
        let mut participants = Vec::new();
        
        for (idx, c) in player_team.iter().enumerate() {
            if c.is_alive() {
                participants.push((idx, c.speed, 0));
            }
        }
        for (idx, c) in enemy_team.iter().enumerate() {
            if c.is_alive() {
                participants.push((idx, c.speed, 1));
            }
        }
        
        if participants.is_empty() {
            return !Self::is_team_alive(player_team) && Self::is_team_alive(enemy_team);
        }
        
        participants.sort_by(|a, b| b.1.cmp(&a.1));

        for (idx, _speed, team) in participants {
            if team == 0 {
                if idx >= player_team.len() || !player_team[idx].is_alive() {
                    continue;
                }
                if player_team[idx].is_frozen() {
                    println!("❄️ {} beku, skip turn!", player_team[idx].name);
                    continue;
                }
                let action_done = Self::player_turn(idx, player_team, enemy_team, turn);
                if !action_done {
                    continue;
                }
                if !Self::is_team_alive(enemy_team) { 
                    return true; 
                }
            } else {
                if idx >= enemy_team.len() || !enemy_team[idx].is_alive() {
                    continue;
                }
                if enemy_team[idx].is_frozen() {
                    println!("❄️ {} beku, skip turn!", enemy_team[idx].name);
                    wait_for_enter_simple();
                    continue;
                }
                
                println!("\n{}⚔️ GILIRAN MUSUH ⚔️{}\n", crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
                display_team_status(player_team, "TIM PEMAIN", crate::core::config::COLOR_CYAN);
                display_team_status(enemy_team, "MUSUH", crate::core::config::COLOR_RED);
                
                EnemyAI::turn(&mut enemy_team[idx], player_team);
                wait_for_enter_simple();
                
                if !Self::is_team_alive(player_team) { 
                    return false; 
                }
            }
        }

        for c in player_team.iter_mut() {
            c.current_energy = (c.current_energy + ENERGY_PER_TURN).min(ULTIMATE_ENERGY_COST);
            c.tick_status_effects();
        }
        for c in enemy_team.iter_mut() {
            c.current_energy = (c.current_energy + ENERGY_PER_TURN).min(ULTIMATE_ENERGY_COST);
            c.tick_status_effects();
        }
        
        if !Self::is_team_alive(enemy_team) {
            println!("💀 Musuh mati karena efek status!");
            wait_for_enter_simple();
            return true;
        }
        if !Self::is_team_alive(player_team) {
            println!("💀 Tim player mati karena efek status!");
            wait_for_enter_simple();
            return false;
        }
        
        false
    }

    fn get_skill_label(skill: &crate::entity::skill::Skill) -> String {
        if skill.is_heal() {
            return "[HEAL]".to_string();
        }
        match &skill.effect {
            Some(Effect::Burn(_)) => "[DAMAGE+BURN]".to_string(),
            Some(Effect::Freeze) => "[DAMAGE+FREEZE]".to_string(),
            Some(Effect::BuffAtk(_)) => "[BUFF]".to_string(),
            Some(Effect::DebuffDef(_)) => "[DEBUFF]".to_string(),
            Some(Effect::Heal(_)) => "[HEAL]".to_string(),
            None => {
                if skill.power > 0 {
                    "[DAMAGE]".to_string()
                } else {
                    "".to_string()
                }
            }
        }
    }

    fn player_turn(character_idx: usize, allies: &mut [Character], enemies: &mut [Character], turn: u32) -> bool {
        battle_header(turn);
        display_team_status(allies, "TIM PEMAIN", crate::core::config::COLOR_CYAN);
        display_team_status(enemies, "MUSUH", crate::core::config::COLOR_RED);
        
        {
            let c = &allies[character_idx];
            let energy_color = if c.current_energy >= ULTIMATE_ENERGY_COST { "\x1b[33m" } else { "\x1b[36m" };
            println!("\n\x1b[36m🔹 Giliran: {} [Energy: {}{}/{}\x1b[36m]\x1b[0m", 
                c.name, energy_color, c.current_energy, ULTIMATE_ENERGY_COST);
            display_hp_bar(&c.name, c.hp, c.max_hp);
        }

        loop {
            println!("┌─────────────────────────┐");
            println!("│ 1. ⚔️ Serangan Dasar     │");
            println!("│ 2. ✨ Skill             │");
            println!("│ 3. 💥 Ultimate          │");
            println!("│ 4. 🛡️ Bertahan          │");
            println!("│ 0. 💾 Save Game (.sv)   │");
            println!("└─────────────────────────┘");
            let choice = get_input("Pilih aksi [0-4]: ");
            
            match choice.as_str() {
                "0" => {
                    println!("💾 Menyimpan game...");
                    if let Some(state_rc) = get_battle_state() {
                        let state = state_rc.borrow();
                        try_save_game(&state);
                        let battle_data = BattleSaveData {
                            turn: turn,
                            player_team: allies.to_vec(),
                            enemy_team: enemies.to_vec(),
                            current_character_idx: character_idx,
                        };
                        if let Some(slot) = crate::core::save_manager::choose_save_slot() {
                            let _ = crate::core::save_manager::save_battle_state(slot, &battle_data);
                        }
                        println!("✅ Game dan battle state berhasil disimpan!");
                    } else {
                        println!("⚠️ Tidak dapat menyimpan game: state tidak tersedia.");
                    }
                    continue;
                }
                "1" => {
                    loop {
                        match Self::select_target_with_cancel(enemies, "musuh") {
                            Some(Ok(target_idx)) => {
                                let (character, target) = Self::get_two_mut_from_different_slices(allies, enemies, character_idx, target_idx, true);
                                character.basic_attack(target, &mut rand::thread_rng());
                                clear_screen();
                                return true;
                            }
                            Some(Err(_)) => {
                                println!("❌ Batal memilih target. Kembali ke menu aksi.");
                                break;
                            }
                            None => continue,
                        }
                    }
                }
                "2" => {
                    let skill_list = allies[character_idx].skills.clone();
                    if skill_list.is_empty() {
                        println!("❌ Tidak punya skill.");
                        continue;
                    }
                    
                    'skill_loop: loop {
                        println!("\n📜 DAFTAR SKILL:");
                        for (i, s) in skill_list.iter().enumerate() {
                            let cost_color = if allies[character_idx].current_energy >= s.cost { "\x1b[32m" } else { "\x1b[31m" };
                            let label = Self::get_skill_label(s);
                            println!("  {}. {:<20}{:<13} ⚡{}{}\x1b[0m", i+1, s.name, label, cost_color, s.cost);
                            println!("     \x1b[90m{}\x1b[0m", s.description);
                        }
                        println!("  0. ↩️ Batal (kembali ke menu aksi)");
                        
                        let skill_idx = get_usize("Pilih skill [0-{}]: ", skill_list.len());
                        if skill_idx == 0 {
                            println!("❌ Batal memilih skill. Kembali ke menu aksi.");
                            break 'skill_loop;
                        }
                        let skill = skill_list[skill_idx - 1].clone();

                        if allies[character_idx].current_energy < skill.cost {
                            println!("❌ Energy tidak cukup! Butuh {} energy.", skill.cost);
                            continue;
                        }

                        if skill.is_heal() {
                            'target_loop: loop {
                                match Self::select_target_with_cancel(allies, "sekutu") {
                                    Some(Ok(target_idx)) => {
                                        let character = &mut allies[character_idx];
                                        if target_idx == character_idx {
                                            character.current_energy -= skill.cost;
                                            if let Some(Effect::Heal(amt)) = &skill.effect {
                                                let heal = (*amt as i32) + skill.power;
                                                character.heal(heal);
                                                println!("💚 {} menyembuhkan dirinya sendiri {} HP.", character.name, heal);
                                            } else {
                                                let target_ptr = character as *mut Character;
                                                unsafe { character.use_skill(&skill, &mut *target_ptr, &mut rand::thread_rng()); }
                                            }
                                            clear_screen();
                                            return true;
                                        } else {
                                            let (character, target) = Self::get_two_mut_from_same_slice(allies, character_idx, target_idx);
                                            character.current_energy -= skill.cost;
                                            character.use_skill(&skill, target, &mut rand::thread_rng());
                                            clear_screen();
                                            return true;
                                        }
                                    }
                                    Some(Err(_)) => {
                                        println!("❌ Batal memilih target. Kembali ke daftar skill.");
                                        break 'target_loop;
                                    }
                                    None => continue,
                                }
                            }
                        } else {
                            'target_loop: loop {
                                match Self::select_target_with_cancel(enemies, "musuh") {
                                    Some(Ok(target_idx)) => {
                                        let (character, target) = Self::get_two_mut_from_different_slices(allies, enemies, character_idx, target_idx, true);
                                        character.current_energy -= skill.cost;
                                        character.use_skill(&skill, target, &mut rand::thread_rng());
                                        clear_screen();
                                        return true;
                                    }
                                    Some(Err(_)) => {
                                        println!("❌ Batal memilih target. Kembali ke daftar skill.");
                                        break 'target_loop;
                                    }
                                    None => continue,
                                }
                            }
                        }
                    }
                }
                "3" => {
                    if allies[character_idx].current_energy >= ULTIMATE_ENERGY_COST {
                        let is_heal = allies[character_idx].ultimate.is_heal();
                        
                        if is_heal {
                            loop {
                                match Self::select_target_with_cancel(allies, "sekutu") {
                                    Some(Ok(target_idx)) => {
                                        let character = &mut allies[character_idx];
                                        if target_idx == character_idx {
                                            character.current_energy = 0;
                                            if let Some(Effect::Heal(amt)) = &character.ultimate.effect {
                                                let heal = (*amt as i32) + character.ultimate.power;
                                                character.heal(heal);
                                                println!("💚 {} menggunakan ultimate pada dirinya sendiri! Sembuh {} HP.", character.name, heal);
                                            } else {
                                                let target_ptr = character as *mut Character;
                                                unsafe { character.use_ultimate(&mut *target_ptr, &mut rand::thread_rng()); }
                                            }
                                            clear_screen();
                                            return true;
                                        } else {
                                            let (character, target) = Self::get_two_mut_from_same_slice(allies, character_idx, target_idx);
                                            character.current_energy = 0;
                                            character.use_ultimate(target, &mut rand::thread_rng());
                                            clear_screen();
                                            return true;
                                        }
                                    }
                                    Some(Err(_)) => {
                                        println!("❌ Batal memilih target. Kembali ke menu aksi.");
                                        break;
                                    }
                                    None => continue,
                                }
                            }
                        } else {
                            loop {
                                match Self::select_target_with_cancel(enemies, "musuh") {
                                    Some(Ok(target_idx)) => {
                                        let (character, target) = Self::get_two_mut_from_different_slices(allies, enemies, character_idx, target_idx, true);
                                        character.current_energy = 0;
                                        character.use_ultimate(target, &mut rand::thread_rng());
                                        clear_screen();
                                        return true;
                                    }
                                    Some(Err(_)) => {
                                        println!("❌ Batal memilih target. Kembali ke menu aksi.");
                                        break;
                                    }
                                    None => continue,
                                }
                            }
                        }
                    } else {
                        println!("❌ Energy kurang! Butuh {} energy.", ULTIMATE_ENERGY_COST);
                    }
                }
                "4" => {
                    println!("🛡️ {} bersiap bertahan! Damage berikutnya -50%.", allies[character_idx].name);
                    let confirm = get_input("Konfirmasi bertahan? (y/n): ");
                    if confirm.to_lowercase() == "y" {
                        allies[character_idx].is_defending = true;
                        clear_screen();
                        return true;
                    } else {
                        println!("❌ Bertahan dibatalkan. Kembali ke menu aksi.");
                        continue;
                    }
                }
                _ => println!("❌ Pilihan tidak valid. Masukkan 0-4."),
            }
        }
    }

    fn select_target_with_cancel(team: &[Character], label: &str) -> Option<Result<usize, ()>> {
        let alive: Vec<usize> = team.iter().enumerate().filter(|(_, c)| c.is_alive()).map(|(i,_)| i).collect();
        if alive.is_empty() {
            println!("Tidak ada {} yang hidup!", label);
            return None;
        }
        println!("\n🎯 PILIH TARGET ({})", label);
        for (j, &idx) in alive.iter().enumerate() {
            let c = &team[idx];
            let hp_color = if c.hp as f32 / c.max_hp as f32 > 0.5 { "\x1b[32m" } 
                           else if c.hp as f32 / c.max_hp as f32 > 0.25 { "\x1b[33m" } 
                           else { "\x1b[31m" };
            println!("  {}. {:<12} ❤️{}{:<4}/{:<4}\x1b[0m 🔮{}", 
                j+1, c.name, hp_color, c.hp, c.max_hp, c.element.name());
        }
        println!("  0. ↩️ Batal");
        
        let choice = get_usize(&format!("Target [0-{}]: ", alive.len()), alive.len());
        if choice == 0 {
            return Some(Err(()));
        }
        Some(Ok(alive[choice - 1]))
    }

    fn get_two_mut_from_different_slices<'a>(
        allies: &'a mut [Character],
        enemies: &'a mut [Character],
        char_idx: usize,
        target_idx: usize,
        is_character_first: bool,
    ) -> (&'a mut Character, &'a mut Character) {
        if is_character_first {
            (&mut allies[char_idx], &mut enemies[target_idx])
        } else {
            (&mut enemies[target_idx], &mut allies[char_idx])
        }
    }

    fn get_two_mut_from_same_slice<'a>(
        slice: &'a mut [Character],
        idx1: usize,
        idx2: usize,
    ) -> (&'a mut Character, &'a mut Character) {
        if idx1 < idx2 {
            let (left, right) = slice.split_at_mut(idx2);
            (&mut left[idx1], &mut right[0])
        } else {
            let (left, right) = slice.split_at_mut(idx1);
            (&mut right[0], &mut left[idx2])
        }
    }

    fn is_team_alive(team: &[Character]) -> bool {
        team.iter().any(|c| c.is_alive())
    }
}
