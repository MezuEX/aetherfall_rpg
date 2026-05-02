use crate::entity::character::Character;
use crate::utils::random::random_bool;

pub struct EnemyAI;

impl EnemyAI {
    pub fn turn(enemy: &mut Character, player_team: &mut [Character]) {
        println!("\n{}👾 Giliran musuh: {}{}", 
            crate::core::config::COLOR_RED, enemy.name, crate::core::config::COLOR_RESET);
        
        if enemy.is_frozen() {
            println!("{}❄️ {} beku, skip turn!{}", 
                crate::core::config::COLOR_CYAN, enemy.name, crate::core::config::COLOR_RESET);
            return;
        }

        // BOSS PHASE: jika HP < 50% dan musuh adalah boss
        let is_boss = enemy.name.contains("Void Knight") || enemy.name.contains("Null Sovereign") || 
                      enemy.name.contains("Ancient Wyrm") || enemy.name.contains("Chaos Beast");
        let hp_ratio = enemy.hp as f32 / enemy.max_hp as f32;
        let is_phase2 = is_boss && hp_ratio < 0.5;
        
        if is_phase2 {
            println!("{}⚠️ BOSS PHASE 2! {} mengamuk!{}", 
                crate::core::config::COLOR_RED, enemy.name, crate::core::config::COLOR_RESET);
            // AI lebih agresif: lebih sering pakai skill/ultimate
        }

        // Cek apakah perlu bertahan (HP rendah)
        if enemy.hp < enemy.max_hp / 3 && random_bool(0.4) && !is_phase2 {
            enemy.is_defending = true;
            println!("🛡️ {} bersiap bertahan!", enemy.name);
            return;
        }
        
        let target = Self::select_target(player_team, enemy);
        if let Some(t) = target {
            let mut rng = rand::thread_rng();
            // Priority ultimate in phase 2
            let ult_chance = if is_phase2 { 0.6 } else { 0.3 };
            if enemy.current_energy >= 100 && random_bool(ult_chance) {
                println!("{}💥 Musuh menggunakan ULTIMATE!{}", crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
                enemy.use_ultimate(t, &mut rng);
                enemy.current_energy = 0;
            }
            else if !enemy.skills.is_empty() && enemy.current_energy >= 30 && random_bool(if is_phase2 { 0.7 } else { 0.4 }) {
                let skill = enemy.skills[0].clone();
                enemy.current_energy -= skill.cost;
                enemy.use_skill(&skill, t, &mut rng);
            } else {
                enemy.basic_attack(t, &mut rng);
            }
        }
    }

    fn select_target<'a>(team: &'a mut [Character], enemy: &Character) -> Option<&'a mut Character> {
        let alive: Vec<usize> = team.iter().enumerate().filter(|(_, c)| c.is_alive()).map(|(i,_)| i).collect();
        if alive.is_empty() { return None; }
        
        let mut best_idx = alive[0];
        let mut best_score = -1000;
        for &idx in &alive {
            let mut score = -team[idx].hp; // prioritas HP rendah
            let mult = enemy.element.advantage(team[idx].element);
            if mult > 1.0 { score += 50; }
            if team[idx].role.name() == "Penyembuh" { score += 30; }
            if team[idx].role.name() == "Penyerang" { score += 10; }
            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }
        Some(&mut team[best_idx])
    }
}
