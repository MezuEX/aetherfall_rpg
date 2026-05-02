use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub unlocked_characters: Vec<String>,
    pub active_team: Vec<String>,
    pub current_chapter: u32,
    pub endings_unlocked: Vec<String>,
    pub global_flags: HashMap<String, bool>,
    pub affinity: HashMap<String, HashMap<String, i32>>,
    pub player_choices: Vec<(String, String)>,
    pub relationship_events_triggered: Vec<String>,
    pub equipment: HashMap<String, Vec<Equipment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub name: String,
    pub slot: EquipmentSlot,
    pub atk_bonus: i32,
    pub def_bonus: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Artifact,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            unlocked_characters: vec!["Aiden".to_string(), "Lyra".to_string(), "Kael".to_string(), "Mira".to_string()],
            active_team: vec!["Aiden".to_string(), "Lyra".to_string(), "Kael".to_string(), "Mira".to_string()],
            current_chapter: 1,
            endings_unlocked: vec![],
            global_flags: HashMap::new(),
            affinity: HashMap::new(),
            player_choices: vec![],
            relationship_events_triggered: vec![],
            equipment: HashMap::new(),
        }
    }

    pub fn add_choice(&mut self, chapter: &str, choice: &str) {
        self.player_choices.push((chapter.to_string(), choice.to_string()));
    }

    #[allow(dead_code)]
    pub fn has_choice(&self, chapter: &str, choice: &str) -> bool {
        self.player_choices.iter().any(|(c, ch)| c == chapter && ch == choice)
    }

    pub fn add_affinity(&mut self, c1: &str, c2: &str, value: i32) {
        let entry = self.affinity.entry(c1.to_string()).or_insert_with(HashMap::new);
        *entry.entry(c2.to_string()).or_insert(0) += value;
        println!("\n{}❤️ Hubungan {} dan {} meningkat +{}!{}", 
            crate::core::config::COLOR_MAGENTA, c1, c2, value, crate::core::config::COLOR_RESET);
        
        let total = self.get_affinity(c1, c2);
        if total >= 30 && !self.relationship_events_triggered.contains(&format!("{}_{}", c1, c2)) {
            self.relationship_events_triggered.push(format!("{}_{}", c1, c2));
            println!("{}✨ Event hubungan khusus antara {} dan {} telah terbuka!{}", 
                crate::core::config::COLOR_YELLOW, c1, c2, crate::core::config::COLOR_RESET);
        }
    }

    pub fn get_affinity(&self, c1: &str, c2: &str) -> i32 {
        self.affinity.get(c1).and_then(|m| m.get(c2)).copied().unwrap_or(0)
    }
    
    pub fn get_affinity_level(&self, c1: &str, c2: &str) -> &'static str {
        let val = self.get_affinity(c1, c2);
        if val >= 50 { "❤️ Soulmate" }
        else if val >= 30 { "💛 Close Friend" }
        else if val >= 15 { "💚 Friendly" }
        else if val >= 5 { "💙 Acquainted" }
        else { "🖤 Stranger" }
    }

    pub fn equip_item(&mut self, character_name: &str, equipment: Equipment) {
        let entry = self.equipment.entry(character_name.to_string()).or_insert_with(Vec::new);
        entry.retain(|e| e.slot != equipment.slot);
        let equip_name = equipment.name.clone();
        entry.push(equipment);
        println!("✅ {} dipasangkan ke {}.", equip_name, character_name);
    }

    pub fn get_equipment_bonus(&self, character_name: &str) -> (i32, i32) {
        let mut atk = 0;
        let mut def = 0;
        if let Some(equips) = self.equipment.get(character_name) {
            for e in equips {
                atk += e.atk_bonus;
                def += e.def_bonus;
            }
        }
        (atk, def)
    }
}
