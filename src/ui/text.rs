pub fn title() {
    println!("{}", crate::core::config::COLOR_MAGENTA);
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                                                          ║");
    println!("║    █████╗ ███████╗████████╗██╗  ██╗███████╗██████╗       ║");
    println!("║   ██╔══██╗██╔════╝╚══██╔══╝██║  ██║██╔════╝██╔══██╗      ║");
    println!("║   ███████║█████╗     ██║   ███████║█████╗  ██████╔╝      ║");
    println!("║   ██╔══██║██╔══╝     ██║   ██╔══██║██╔══╝  ██╔══██╗      ║");
    println!("║   ██║  ██║███████╗   ██║   ██║  ██║███████╗██║  ██║      ║");
    println!("║   ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝      ║");
    println!("║                                                          ║");
    println!("║              TURN-BASED FANTASY BATTLE RPG                ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("{}", crate::core::config::COLOR_RESET);
}

pub fn show_team_builder_prompt() {
    println!("\n{}[TEAM BUILDER]{}", crate::core::config::COLOR_CYAN, crate::core::config::COLOR_RESET);
    println!("Anda dapat mengatur komposisi tim sebelum battle.");
    println!("Saat ini tim aktif: 4 karakter pertama dari roster.");
    println!("(Fitur pengaturan tim akan datang di update selanjutnya!)");
}

pub fn show_affinity_panel(state: &crate::core::state::GameState, characters: &[crate::entity::character::Character]) {
    println!("\n{}═══════════ AFFINITY STATUS ═══════════{}", 
        crate::core::config::COLOR_MAGENTA, crate::core::config::COLOR_RESET);
    for i in 0..characters.len() {
        for j in i+1..characters.len() {
            let affinity = state.get_affinity(&characters[i].name, &characters[j].name);
            let level = state.get_affinity_level(&characters[i].name, &characters[j].name);
            println!("  {} & {} : {} ({} pts)", 
                characters[i].name, characters[j].name, level, affinity);
        }
    }
}
