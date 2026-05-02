use crate::entity::character::Character;

pub fn display_hp_bar(name: &str, current: i32, max: i32) {
    let percent = (current as f32 / max as f32 * 20.0) as usize;
    let color = if current as f32 / max as f32 > 0.5 {
        "\x1b[32m"
    } else if current as f32 / max as f32 > 0.25 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };
    let bar = "█".repeat(percent) + &"░".repeat(20 - percent);
    if name.is_empty() {
        println!("   {color}[{}]\x1b[0m {}/{}", bar, current, max);
    } else {
        println!("{}: {color}[{}]\x1b[0m {}/{}", name, bar, current, max);
    }
}

pub fn display_team_status(team: &[Character], label: &str, color: &str) {
    let border = "═".repeat(40);
    println!("\n{}{} {} {}{}", color, border, label, border, "\x1b[0m");
    
    for (i, c) in team.iter().enumerate() {
        if c.is_alive() {
            let hp_color = if c.hp as f32 / c.max_hp as f32 > 0.5 { "\x1b[32m" } 
                           else if c.hp as f32 / c.max_hp as f32 > 0.25 { "\x1b[33m" } 
                           else { "\x1b[31m" };
            
            let energy_bar_len = (c.current_energy as f32 / 100.0 * 10.0) as usize;
            let energy_bar = "⚡".repeat(energy_bar_len) + &"·".repeat(10 - energy_bar_len);
            
            println!("{}. {:<12} ❤️{}{:<4}/{:<4}\x1b[0m {} 🔮{}", 
                i+1, c.name, hp_color, c.hp, c.max_hp, energy_bar, c.element.name());
        } else {
            println!("{}. {:<12} \x1b[90m💀 KNOCKED OUT\x1b[0m", i+1, c.name);
        }
    }
}

pub fn battle_header(turn: u32) {
    let border = "═".repeat(50);
    println!("\n\x1b[35m{} BATTLE TURN {} {}\x1b[0m", border, turn, border);
}

pub fn battle_start_header() {
    let border = "═".repeat(50);
    println!("\n\x1b[33m{} BATTLE START {}\x1b[0m", border, border);
}

pub fn battle_result(win: bool) {
    let border = "═".repeat(50);
    if win {
        println!("\n\x1b[32m{} VICTORY! {}\x1b[0m", border, border);
    } else {
        println!("\n\x1b[31m{} DEFEAT... {}\x1b[0m", border, border);
    }
}

pub fn damage_animation(target_name: &str, damage: i32, is_critical: bool) {
    if is_critical {
        println!("\n{}💥 CRITICAL HIT! 💥{}", crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
        println!("{}╔══════════════════════════════════════╗{}", 
            crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
        println!("{}║  {} -{} HP!{}                ║{}", 
            crate::core::config::COLOR_RED, target_name, damage, crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
        println!("{}╚══════════════════════════════════════╝{}", 
            crate::core::config::COLOR_RED, crate::core::config::COLOR_RESET);
    } else {
        println!("{}  ✨ {} -{} HP! ✨{}", crate::core::config::COLOR_YELLOW, target_name, damage, crate::core::config::COLOR_RESET);
    }
}

pub fn heal_animation(target_name: &str, amount: i32) {
    println!("\n{}💚 HEAL! 💚{}", crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
    println!("{}╔══════════════════════════════════════╗{}", 
        crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
    println!("{}║  {} +{} HP!{}                      ║{}", 
        crate::core::config::COLOR_GREEN, target_name, amount, crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
    println!("{}╚══════════════════════════════════════╝{}", 
        crate::core::config::COLOR_GREEN, crate::core::config::COLOR_RESET);
}

pub fn status_effect_animation(target_name: &str, effect_name: &str, color: &str) {
    println!("{}{}🌀 {} terkena {}! 🌀{}", 
        color, crate::core::config::COLOR_RESET, target_name, effect_name, crate::core::config::COLOR_RESET);
}
