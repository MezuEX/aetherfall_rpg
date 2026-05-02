use crate::entity::character::Character;
use crate::battle::ai::EnemyAI;
use crate::core::config::{ENERGY_PER_TURN, ULTIMATE_ENERGY_COST};
use crate::utils::input::{get_input, get_usize};
use crate::ui::battle_ui::display_hp_bar;

pub struct TurnManager;

impl TurnManager {
    pub fn run_turn(player_team: &mut Vec<Character>, enemy_team: &mut Vec<Character>) -> bool {
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
        participants.sort_by(|a, b| b.1.cmp(&a.1));

        for (idx, _speed, team) in participants {
            if team == 0 {
                if player_team[idx].is_frozen() {
                    println!("❄️ {} beku, skip turn!", player_team[idx].name);
                    continue;
                }
                Self::player_turn(idx, player_team, enemy_team);
                if !Self::is_team_alive(enemy_team) { return true; }
            } else {
                if enemy_team[idx].is_frozen() {
                    println!("❄️ {} beku, skip turn!", enemy_team[idx].name);
                    continue;
                }
                EnemyAI::turn(&mut enemy_team[idx], player_team);
                if !Self::is_team_alive(player_team) { return false; }
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
        Self::is_team_alive(enemy_team)
    }

    fn player_turn(character_idx: usize, allies: &mut [Character], enemies: &mut [Character]) {
        // Tampilkan status (hanya baca)
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
            println!("└─────────────────────────┘");
            let choice = get_input("Pilih aksi [1-4]: ");
            match choice.as_str() {
                "1" => {
                    if let Some(target_idx) = Self::select_target_index(enemies, "musuh") {
                        let (character, target) = Self::get_two_mut_from_different_slices(allies, enemies, character_idx, target_idx, true);
                        character.basic_attack(target, &mut rand::thread_rng());
                        break;
                    }
                }
                "2" => {
                    let skill_list = allies[character_idx].skills.clone();
                    if skill_list.is_empty() {
                        println!("❌ Tidak punya skill.");
                        continue;
                    }
                    println!("\n📜 DAFTAR SKILL:");
                    for (i, s) in skill_list.iter().enumerate() {
                        let cost_color = if allies[character_idx].current_energy >= s.cost { "\x1b[32m" } else { "\x1b[31m" };
                        let heal_tag = if s.is_heal() { " [HEAL]" } else { "" };
                        println!("  {}. {:<20}{} ⚡{}{}\x1b[0m", i+1, s.name, heal_tag, cost_color, s.cost);
                    }
                    let skill_idx = get_usize("Pilih skill [1-{}]: ", skill_list.len()) - 1;
                    let skill = skill_list[skill_idx].clone();

                    if allies[character_idx].current_energy < skill.cost {
                        println!("❌ Energy tidak cukup! Butuh {} energy.", skill.cost);
                        continue;
                    }

                    if skill.is_heal() {
                        // Target di allies (bisa self atau lain)
                        if let Some(target_idx) = Self::select_target_index(allies, "sekutu") {
                            let (character, target) = Self::get_two_mut_from_same_slice(allies, character_idx, target_idx);
                            character.current_energy -= skill.cost;
                            character.use_skill(&skill, target, &mut rand::thread_rng());
                            break;
                        }
                    } else {
                        // Target di enemies
                        if let Some(target_idx) = Self::select_target_index(enemies, "musuh") {
                            let (character, target) = Self::get_two_mut_from_different_slices(allies, enemies, character_idx, target_idx, true);
                            character.current_energy -= skill.cost;
                            character.use_skill(&skill, target, &mut rand::thread_rng());
                            break;
                        }
                    }
                }
                "3" => {
                    if allies[character_idx].current_energy >= ULTIMATE_ENERGY_COST {
                        let is_heal = allies[character_idx].ultimate.is_heal();
                        if is_heal {
                            if let Some(target_idx) = Self::select_target_index(allies, "sekutu") {
                                let (character, target) = Self::get_two_mut_from_same_slice(allies, character_idx, target_idx);
                                character.use_ultimate(target, &mut rand::thread_rng());
                                character.current_energy = 0;
                                break;
                            }
                        } else {
                            if let Some(target_idx) = Self::select_target_index(enemies, "musuh") {
                                let (character, target) = Self::get_two_mut_from_different_slices(allies, enemies, character_idx, target_idx, true);
                                character.use_ultimate(target, &mut rand::thread_rng());
                                character.current_energy = 0;
                                break;
                            }
                        }
                    } else {
                        println!("❌ Energy kurang! Butuh {} energy.", ULTIMATE_ENERGY_COST);
                    }
                }
                "4" => {
                    allies[character_idx].is_defending = true;
                    println!("🛡️ {} bersiap bertahan! Damage berikutnya -50%.", allies[character_idx].name);
                    break;
                }
                _ => println!("❌ Pilihan tidak valid. Masukkan 1-4."),
            }
        }
    }

    /// Cari indeks target yang hidup
    fn select_target_index(team: &[Character], label: &str) -> Option<usize> {
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
        let choice = get_usize(&format!("Target [1-{}]: ", alive.len()), alive.len()) - 1;
        Some(alive[choice])
    }

    /// Mengambil dua mutable reference dari slice yang BERBEDA (allies dan enemies)
    /// `is_character_first` menentukan urutan return (character, target)
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

    /// Mengambil dua mutable reference dari slice yang SAMA (allies) dengan indeks berbeda
    /// Gunakan split_at_mut untuk menghindari double borrow
    fn get_two_mut_from_same_slice<'a>(
        slice: &'a mut [Character],
        idx1: usize,
        idx2: usize,
    ) -> (&'a mut Character, &'a mut Character) {
        assert!(idx1 != idx2);
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
