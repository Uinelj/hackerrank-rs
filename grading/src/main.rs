use std::io;
use std::cmp::Ordering;

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

fn is_grade_valid(grade: u32) -> bool {
    match grade {
        0...100 => return true,
        _ => return false
    }
}

fn apply_grading_rules(grade: u32) -> u32 {
    let mut modified_grade = grade;
    match modified_grade {
        0...37 => return modified_grade,
        _ => {
            if modified_grade%5 >= 3 { 
                modified_grade += 5 - modified_grade%5;
            }
            return modified_grade;
       },
    }
}

fn main() {
    let n_students = get_input_u32();
    let mut students_grades = Vec::new();
    let mut grade_buffer : u32;

    for _ in 0..n_students {
        students_grades.push(get_input_u32());
    }
    
    let regularized_grades = students_grades.into_iter()
                   .map(|grade|{ apply_grading_rules(grade) });
    
    for grade in regularized_grades {
        println!("{}", grade)
    }
}
