use crate::core::state::GameState;
use crate::utils::input::wait_for_enter;

pub fn trigger_relationship_event(state: &mut GameState, char1: &str, char2: &str) {
    let key = format!("{}_{}", char1, char2);
    if state.relationship_events_triggered.contains(&key) {
        return;
    }
    
    match (char1, char2) {
        ("Aiden", "Lyra") => {
            println!("\n{}┌─ [Aiden & Lyra]{}", crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
            println!("Lyra: \"Aiden, kau terlalu gegabah! Teruskan saja tanpa berpikir.\"");
            println!("Aiden: \"Dan kau terlalu ragu-ragu! Tanpa tindakan, kita tidak akan pernah maju.\"");
            println!("{}Mereka berdua saling memahami setelah bertengkar sebentar.{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!("✨ Bonus synergy: +5% crit chance untuk semua tim!");
            state.add_affinity(char1, char2, 10);
            state.relationship_events_triggered.push(key);
            wait_for_enter();
        }
        ("Elara", "Vex") => {
            println!("\n{}┌─ [Elara & Vex]{}", crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
            println!("Elara: \"Masih ada harapan, Vex. Kau tidak sendiri.\"");
            println!("Vex: \"Harapan... itu hanya ilusi yang diciptakan untuk menenangkan rasa takut.\"");
            println!("Elara: \"Tapi aku di sini. Itu nyata.\"");
            println!("{}Vex terdiam. Mungkin dia mulai percaya.{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!("✨ Bonus: Serangan kegelapan Vex mendapat tambahan cahaya.");
            state.add_affinity(char1, char2, 15);
            state.relationship_events_triggered.push(key);
            wait_for_enter();
        }
        ("Kael", "Terra") => {
            println!("\n{}┌─ [Kael & Terra]{}", crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
            println!("Terra: \"Kael, perisaimu menginspirasiku. Aku ingin belajar darimu.\"");
            println!("Kael: \"Tanah tidak pernah bergeming. Itu rahasia kekuatanku.\"");
            println!("{}Mereka berlatih bersama dan menjadi lebih kuat.{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!("✨ Bonus DEF +10 untuk semua anggota tim.");
            state.add_affinity(char1, char2, 10);
            state.relationship_events_triggered.push(key);
            wait_for_enter();
        }
        ("Mira", "Zeph") => {
            println!("\n{}┌─ [Mira & Zeph]{}", crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
            println!("Zeph: \"Angin membawa kebebasan, Mira. Kau mengerti itu.\"");
            println!("Mira: \"Ya, dan bersama kita bisa terbang lebih tinggi.\"");
            println!("{}Mereka menari bersama angin.{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!("✨ Bonus Speed +10 untuk semua anggota tim.");
            state.add_affinity(char1, char2, 10);
            state.relationship_events_triggered.push(key);
            wait_for_enter();
        }
        ("Orion", "Nyx") => {
            println!("\n{}┌─ [Orion & Nyx]{}", crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
            println!("Orion: \"Waktu dan kekacauan adalah dua sisi koin yang sama.\"");
            println!("Nyx: \"Dan kita berdua adalah penjaganya.\"");
            println!("{}Mereka menemukan harmoni dalam perbedaan.{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
            println!("✨ Bonus: Ultimate charge +20% lebih cepat.");
            state.add_affinity(char1, char2, 15);
            state.relationship_events_triggered.push(key);
            wait_for_enter();
        }
        _ => {}
    }
}

pub fn check_all_relationship_events(state: &mut GameState) {
    let pairs = vec![
        ("Aiden", "Lyra"),
        ("Elara", "Vex"),
        ("Kael", "Terra"),
        ("Mira", "Zeph"),
        ("Orion", "Nyx"),
    ];
    for (c1, c2) in pairs {
        if state.get_affinity(c1, c2) >= 30 {
            trigger_relationship_event(state, c1, c2);
        }
    }
}
