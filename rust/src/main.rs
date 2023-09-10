use std::env;
use std::str;
use std::result::Result;
use std::process::ExitCode;

#[derive(Clone)]
enum Action {
    ADD(char),
    REMOVE(char),
    SUBSTITUTE(char, char),
    IGNORE(char)
}

fn print_trace_long(trace: &Vec<Action>) {
    println!("\nLong Trace:");
    for action in trace {
        match action {
            Action::IGNORE(x) => println!("\tIGNORE '{x}'"),
            Action::ADD(x) => println!("\tADD '{x}'"),
            Action::REMOVE(x) => println!("\tREMOVE '{x}'"),
            Action::SUBSTITUTE(x, y) =>  println!("\tSUBSTITUTE '{x}' with '{y}'")
        };
    };
}

fn print_trace_short(trace: &Vec<Action>) {
    print!("\nShort Trace:\t");
    for action in trace {
        match action {
            Action::IGNORE(x) => print!("{x} "),
            Action::ADD(x) => print!("+{x} "),
            Action::REMOVE(x) => print!("-{x} "),
            Action::SUBSTITUTE(x, y) => print!("( -{x} +{y} ) ")
        };
    };
    println!("\n");
}


fn min(v1: u32, v2: u32, v3: u32) -> u32 {
    let mut smallest: u32 = v1;

    if v2 < smallest { smallest = v2; }
    if v3 < smallest { smallest = v3; }

    return smallest
}


fn trace_action(source: &Vec<char>, destination: &Vec<char>, action_cache: &Vec<Vec<Option<Action>>>) -> Vec<Action> {
    let mut path: Vec<Action> = Vec::new();

    let mut n1: usize = source.len();
    let mut n2: usize = destination.len();

    while (n1 > 0) | (n2 > 0) {
        let action = action_cache[n1][n2].clone().unwrap();

        n1 -= 1; n2 -= 1;

        match action {
            Action::ADD(_) => n1 += 1,
            Action::REMOVE(_) => n2 += 1,
            _ => {}
        };

        path.push(action);
    };

    path.reverse();
    path
}


fn levenshtein(source: &Vec<char>, destination: &Vec<char>) -> u32 {
    let mut distance_cache: Vec<Vec<Option<u32>>> = vec![vec![None; destination.len() + 1]; source.len() + 1];
    let mut actions_cache: Vec<Vec<Option<Action>>> = vec![vec![None; destination.len() + 1]; source.len() + 1];
    
    distance_cache[0][0] = Some(0);
    actions_cache[0][0] = Some(Action::IGNORE('_'));

    for n1 in 1..(source.len()  + 1) {
        distance_cache[n1][0] = Some(n1 as u32);
        actions_cache[n1][0] = Some(Action::REMOVE(source[n1 - 1]));
    }

    for n2 in 1..(destination.len()  + 1) {
        distance_cache[0][n2] = Some(n2 as u32);
        actions_cache[0][n2] = Some(Action::ADD(destination[n2 - 1]));
    }

    for n1 in 1..(source.len() + 1) {
        for n2 in 1..(destination.len() + 1) {
            if source[n1 - 1] == destination[n2 - 1] {
                distance_cache[n1][n2] = distance_cache[n1 - 1][n2 - 1];
                actions_cache[n1][n2] = Some(Action::IGNORE(source[n1 - 1]));
                continue;
            };

            let add_char: u32 = 1 + distance_cache[n1][n2 - 1].unwrap();
            let remove_char: u32 = 1 + distance_cache[n1 - 1][n2].unwrap();
            let substitute_char: u32 = 1 + distance_cache[n1 - 1][n2 - 1].unwrap();

            distance_cache[n1][n2] = Some(min(add_char, remove_char,substitute_char));
            
            actions_cache[n1][n2] = match distance_cache[n1][n2].unwrap() {
                d if d == add_char => Some(Action::ADD(destination[n2 - 1])),
                d if d == remove_char => Some(Action::REMOVE(source[n1 - 1])),
                _ => Some(Action::SUBSTITUTE(source[n1 - 1], destination[n2 - 1])),
            };
        }
    }
    let path = trace_action(source, destination, &actions_cache);
    print_trace_long(&path);
    print_trace_short(&path);

    distance_cache[source.len()][destination.len()].unwrap()
}


fn usage(program: &str, err_msg: &str) {
    eprintln!("{err_msg}");
    eprintln!("Usage: {program} <STRING1> <STRING2>");
    eprintln!("");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args().into_iter();

    let program: String = args.next().unwrap();

    let string1: Vec<char> = args.next().ok_or_else(|| {
        usage(&program, "ERROR: Invalid Args - expected STRING1");
    })?.chars().collect();

    let string2: Vec<char>= args.next().ok_or_else(|| {
        usage(&program, "ERROR: Invalid Args - expected STRING2");
    })?.chars().collect();

    let dist : u32 = levenshtein(&string1, &string2);
    println!("Number of changes: {dist}");

    Ok(())
}

fn main() -> ExitCode {
    return match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE
    };
}

