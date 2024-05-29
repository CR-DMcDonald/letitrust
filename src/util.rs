
pub fn print_banner() {
    println!("");
    println!("██      ███████ ████████     ██ ████████     ██████  ██    ██ ███████ ████████ ");
    println!("██      ██         ██        ██    ██        ██   ██ ██    ██ ██         ██    ");
    println!("██      █████      ██        ██    ██        ██████  ██    ██ ███████    ██    ");
    println!("██      ██         ██        ██    ██        ██   ██ ██    ██      ██    ██    ");
    println!("███████ ███████    ██        ██    ██        ██   ██  ██████  ███████    ██    ");                                                                    
    println!("");                                                                  
    println!("letitrust v0.1");
    println!("Written by Darren McDonald, Cryptic Red");
}

pub fn print_usage() {
    println!("");
    println!("Usage: ./letitrust -f <filename>");
    println!("       ./letitrust -d <domain>");
}

pub fn print_green(text: &str) {
    println!("\x1B[32m{}\x1B[0m", text);
}

pub fn print_yellow(text: &str) {
    println!("\x1B[33m{}\x1B[0m", text);
}

pub fn print_red(text: &str) {
    println!("\x1B[31m{}\x1B[0m", text);
}
