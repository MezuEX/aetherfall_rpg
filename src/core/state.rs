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
        }
    }

    pub fn add_affinity(&mut self, c1: &str, c2: &str, value: i32) {
        let entry = self.affinity.entry(c1.to_string()).or_insert_with(HashMap::new);
        *entry.entry(c2.to_string()).or_insert(0) += value;
        println!("\n{}❤️ Hubungan {} dan {} meningkat +{}!{}", 
            crate::core::config::COLOR_MAGENTA, c1, c2, value, crate::core::config::COLOR_RESET);
        
        let total = self.get_affinity(c1, c2);
        if total >= 30 {
            println!("{}✨ Hubungan {} dan {} sudah sangat dekat! Bonus synergy aktif!{}", 
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
}
