use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    const LENGTH: usize = 30;
    const CARGO: &str = "    [[]]";
    const BASE_MS: f32 = 100.00;
    const ACCELERATION_FACTOR: f32 = 0.085;

    for i in 0..=LENGTH as u64 {
        let number: u64 = i+1;
        print!("\x1b[2J\x1b[H"); //cls and cursor

        let spaces: String = " ".repeat(i as usize);
        let cargo_ready: String = format!("{}{}", spaces, CARGO);
        print!("\x1b[34m");
        println!("{}>", ("=".repeat(LENGTH)));
        print_corresponding_color(i);
        println!("{}", cargo_ready);
        print!("\x1b[34m");
        println!("{}>", ("=".repeat(LENGTH)));
        println!("Riel Foundation");
        print!("\x1b[0m"); 
        io::stdout().flush().unwrap(); //flush buffer

        let number_float: f32 = number as f32;
        let time_accelerated: f32 = BASE_MS / (number_float * ACCELERATION_FACTOR);
        let time: u64 = time_accelerated as u64;
        thread::sleep(Duration::from_millis(time));
    }
}
fn print_corresponding_color(i: u64) {
    if i < 10 {
        print!("\x1b[34m");
        return;
    } else if i < 25 {
        print!("\x1b[36m");
        return;
    } else {
        print!("\x1b[31m");
        return;
    }
}