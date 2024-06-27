use std::{fs::read_to_string, io::stdin};

fn read_line()->String{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Err");
    input
}

struct Returns{
    jellyfindir: String,
    sourcedir: String,
}
fn main() {
    // println!("Hello, world!");
    let display = display();
    println!("{}{}", &display.jellyfindir, &display.sourcedir)
}
fn display()->Returns{
    print!("\x1B[2J\x1B[1;1H");
    let content = read_to_string("menu").expect("Internal error.");
    println!("{}",content);
    println!("Please enter your jellyfin lib dir, then your source folder after return.");
    return Returns{
        jellyfindir:read_line(),
        sourcedir:read_line(),
    }
    
}