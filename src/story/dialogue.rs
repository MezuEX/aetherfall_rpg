pub fn print_dialogue(speaker: &str, text: &str) {
    println!("\n{}┌─ [{}]{}", crate::core::config::COLOR_CYAN, speaker, crate::core::config::COLOR_RESET);
    println!("{}│{} \"{}\"", crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET, text);
    println!("{}└──────────────────────────────────────────{}", crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
}

pub fn narrator(text: &str) {
    println!("\n{}┌─ [Narrator]{}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
    println!("{}│{} {}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET, text);
    println!("{}└──────────────────────────────────────────{}", crate::core::config::COLOR_YELLOW, crate::core::config::COLOR_RESET);
}
