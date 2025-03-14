use std::time;

#[derive(Debug)]
struct PlayerInformation {
    level: u32,
    resource: f64,
    last_save: time::SystemTime
}

fn main() {
    println!("This is the beginning of the code");
    let player_test = PlayerInformation{
        level: 35,
        resource: 100.0,
        last_save: time::SystemTime::now()
    };

    println!("{}", player_test.level);
    println!("{}", player_test.resource);
    println!("{:?}", player_test.last_save);
    
    println!("{:?}", player_test);
}
