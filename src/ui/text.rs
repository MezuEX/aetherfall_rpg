pub fn title() {
    println!("{}", crate::core::config::COLOR_MAGENTA);
    println!("    █████╗ ███████╗████████╗██╗  ██╗███████╗██████╗ ███████╗ █████╗ ██╗     ██╗");
    println!("   ██╔══██╗██╔════╝╚══██╔══╝██║  ██║██╔════╝██╔══██╗██╔════╝██╔══██╗██║     ██║");
    println!("   ███████║█████╗     ██║   ███████║█████╗  ██████╔╝█████╗  ███████║██║     ██║");
    println!("   ██╔══██║██╔══╝     ██║   ██╔══██║██╔══╝  ██╔══██╗██╔══╝  ██╔══██║██║     ██║");
    println!("   ██║  ██║███████╗   ██║   ██║  ██║███████╗██║  ██║██║     ██║  ██║███████╗███████╗");
    println!("   ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝");
    println!();
    println!("                 TURN-BASED FANTASY BATTLE RPG v1.0                    ");
    println!("{}", crate::core::config::COLOR_RESET);
}

pub fn show_synopsis() {
    let border = "═".repeat(50);
    println!("\n{}╔{}╗{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    println!("{}║{:^50}║{}", 
        crate::core::config::COLOR_CYAN, "📖 SINOPSIS CERITA 📖", crate::core::config::COLOR_RESET);
    println!("{}╠{}╣{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "🌍 DUNIA: AETHERFALL", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   Sebuah dunia yang dulu harmonis", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   berkat Core Element Nexus.", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   Tapi 'The Shattering' menghancurkan", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   segalanya. Enam elemen menjadi liar.", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "🧙 PERANMU: WEAVER", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   Kamu adalah satu-satunya yang bisa", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   menyatukan energi elemen.", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   Panggil 12 karakter unik, bina", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   hubungan (affinity), dan pilih", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   nasib dunia di antara 4 ending.", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "⚔️ MUSUH UTAMA: NULL SOVEREIGN", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   Entitas yang ingin menghapus", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   semua elemen dan mengembalikan", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   dunia ke keadaan kosong (null).", crate::core::config::COLOR_RESET);
    println!("{}║{:<50}║{}", 
        crate::core::config::COLOR_CYAN, "   Apakah kau bisa menghentikannya?", crate::core::config::COLOR_RESET);
    println!("{}╚{}╝{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
}

pub fn show_team_builder_prompt() {
    let border = "═".repeat(50);
    println!("\n{}┌{}┐{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    println!("{}│{:^50}│{}", 
        crate::core::config::COLOR_CYAN, "👥 TEAM BUILDER MENU 👥", crate::core::config::COLOR_RESET);
    println!("{}├{}┤{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
    println!("{}│{:<50}│{}", 
        crate::core::config::COLOR_CYAN, "Silakan masuk ke menu Team Builder", crate::core::config::COLOR_RESET);
    println!("{}│{:<50}│{}", 
        crate::core::config::COLOR_CYAN, "dari Main Menu (opsi 3)", crate::core::config::COLOR_RESET);
    println!("{}│{:<50}│{}", 
        crate::core::config::COLOR_CYAN, "Di sana kamu bisa mengatur", crate::core::config::COLOR_RESET);
    println!("{}│{:<50}│{}", 
        crate::core::config::COLOR_CYAN, "komposisi 4 karakter dari roster.", crate::core::config::COLOR_RESET);
    println!("{}└{}┘{}", 
        crate::core::config::COLOR_CYAN, border, crate::core::config::COLOR_RESET);
}

pub fn show_affinity_panel(state: &crate::core::state::GameState, characters: &[crate::entity::character::Character]) {
    let border = "═".repeat(50);
    println!("\n{}┌{}┐{}", 
        crate::core::config::COLOR_MAGENTA, border, crate::core::config::COLOR_RESET);
    println!("{}│{:^50}│{}", 
        crate::core::config::COLOR_MAGENTA, "❤️ AFFINITY STATUS ❤️", crate::core::config::COLOR_RESET);
    println!("{}├{}┤{}", 
        crate::core::config::COLOR_MAGENTA, border, crate::core::config::COLOR_RESET);
    
    let mut has_any = false;
    for i in 0..characters.len() {
        for j in i+1..characters.len() {
            let affinity = state.get_affinity(&characters[i].name, &characters[j].name);
            if affinity > 0 {
                has_any = true;
                let level = state.get_affinity_level(&characters[i].name, &characters[j].name);
                let color = if affinity >= 50 { "\x1b[35m" } 
                           else if affinity >= 30 { "\x1b[33m" } 
                           else { "\x1b[36m" };
                let text = format!("{} ✦ {} → {}{}{} ({} pts)", 
                    characters[i].name, characters[j].name, color, level, crate::core::config::COLOR_RESET, affinity);
                if text.len() <= 48 {
                    println!("{}│ {:<48} │{}", crate::core::config::COLOR_MAGENTA, text, crate::core::config::COLOR_RESET);
                } else {
                    println!("{}│ {:<48} │{}", crate::core::config::COLOR_MAGENTA, &text[0..48], crate::core::config::COLOR_RESET);
                }
            }
        }
    }
    
    if !has_any {
        println!("{}│{:<50}│{}", 
            crate::core::config::COLOR_MAGENTA, "Belum ada hubungan terbentuk", crate::core::config::COLOR_RESET);
        println!("{}│{:<50}│{}", 
            crate::core::config::COLOR_MAGENTA, "Lanjutkan cerita untuk membangun affinity!", crate::core::config::COLOR_RESET);
    }
    
    println!("{}└{}┘{}", 
        crate::core::config::COLOR_MAGENTA, border, crate::core::config::COLOR_RESET);
}
