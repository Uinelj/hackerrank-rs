use std::io;

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok().expect("read error");
    return buffer;
}

fn get_input_u32() -> u32 {
    let buffer = get_input();
    let mut parsed_buffer : u32 = buffer.trim().parse::<u32>().unwrap();
    return parsed_buffer;
}

fn main() {
    let physics_notes = vec![15, 12, 8, 8, 7, 7, 7, 6, 5, 3];
    let history_notes = vec![10, 25, 17, 11, 13, 17, 20, 13, 9, 5];

    // Computing means
    let mut physics_mean : f64 = physics_notes.iter()
                                              .fold(0, |sum, i| sum + i ) as f64;
    physics_mean /= physics_notes.len() as f64;

    let mut history_mean : f64 = history_notes.iter()
                                              .fold(0, |sum, i| sum + i ) as f64;
    history_mean /= history_notes.len() as f64;

    let mut physics_cov = physics_notes.iter().fold(0, |sum, i| {
        sum + (i - physics_mean).pow(2);
    });
    println!("{:?}", physics_mean);
    println!("{:?}", history_mean);
}