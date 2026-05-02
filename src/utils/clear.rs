use std::process::Command;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/c", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

pub fn wait_and_clear() {
    let mut input = String::new();
    println!("\nTekan Enter untuk melanjutkan...");
    std::io::stdin().read_line(&mut input).unwrap();
    clear_screen();
}
