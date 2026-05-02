use crate::entity::character::Character;

pub fn display_hp_bar(name: &str, current: i32, max: i32) {
    let percent = (current as f32 / max as f32 * 20.0) as usize;
    let color = if current as f32 / max as f32 > 0.5 {
        "\x1b[32m" // Hijau
    } else if current as f32 / max as f32 > 0.25 {
        "\x1b[33m" // Kuning
    } else {
        "\x1b[31m" // Merah
    };
    let bar = "█".repeat(percent) + &"░".repeat(20 - percent);
    if name.is_empty() {
        println!("   {color}[{}]\x1b[0m {}/{}", bar, current, max);
    } else {
        println!("{}: {color}[{}]\x1b[0m {}/{}", name, bar, current, max);
    }
}

pub fn display_team_status(team: &[Character], label: &str, color: &str) {
    println!("\n{}{} {}{}", color, "═".repeat(30), label, "\x1b[0m");
    for (i, c) in team.iter().enumerate() {
        if c.is_alive() {
            let hp_color = if c.hp as f32 / c.max_hp as f32 > 0.5 { "\x1b[32m" } 
                           else if c.hp as f32 / c.max_hp as f32 > 0.25 { "\x1b[33m" } 
                           else { "\x1b[31m" };
            println!("{}. {:<12} ❤️{}{:<4}/{:<4}\x1b[0m ⚡{:<3} 🔮{}", 
                i+1, c.name, hp_color, c.hp, c.max_hp, c.current_energy, c.element.name());
        } else {
            println!("{}. {:<12} \x1b[90m💀 KNOCKED OUT\x1b[0m", i+1, c.name);
        }
    }
}

pub fn battle_header(turn: u32) {
    println!("\n\x1b[35m{}═{} BATTLE TURN {} {}═{}\x1b[0m", 
        "═".repeat(10), "═".repeat(10), turn, "═".repeat(10), "═".repeat(10));
}

pub fn battle_result(win: bool) {
    if win {
        println!("\n\x1b[32m{}═{} VICTORY! {}═{}\x1b[0m", 
            "═".repeat(15), "═".repeat(15), "═".repeat(15), "═".repeat(15));
    } else {
        println!("\n\x1b[31m{}═{} DEFEAT... {}═{}\x1b[0m", 
            "═".repeat(15), "═".repeat(15), "═".repeat(15), "═".repeat(15));
    }
}
