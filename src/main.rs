use std::io::stdin;
use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    
    let f: &str = "Some string";
    println!("{}", f);

    if args.len() > 0 {
        match args[0].as_str() {
            "echo" => echo(),
            "cat" => cat(),
            _ => println!("Uknown command"),
        }   
    }

}

fn echo() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read");
}

fn cat() {

}
