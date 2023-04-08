use std::{thread, time};
use colored::Colorize;
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        println!("Installing TF2 Sex Update.    ({}%)", count);
        thread::sleep(time::Duration::from_secs(1));
        if count == 100 {
            println!("{}", "Finished Installing Sex Update!".green().bold());
            break;
        }
        count += 1;
        print!("\x1b[1A");
        print!("\x1b[2K");
        println!("Installing TF2 Sex Update..   ({}%)", count);
        thread::sleep(time::Duration::from_secs(1));
        if count == 100 {
            println!("{}", "Finished Installing Sex Update!".green().bold());
            break;
        }
        count += 1;
        print!("\x1b[1A");
        print!("\x1b[2K");
        println!("Installing TF2 Sex Update...  ({}%)", count);
        thread::sleep(time::Duration::from_secs(1));
        if count == 100 {
            println!("{}", "Finished Installing Sex Update!".green().bold());
            break;
        }
        print!("\x1b[1A");
        print!("\x1b[2K");
    }
}

