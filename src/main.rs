use std::io::Write;

fn main() {
    loop {
        user_input();
    }
}

fn user_input() {
    print!("> ");
    let mut input = String::new();
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).unwrap();

    for word in input.split_whitespace() {
        if word.starts_with('.') {
            handle_input(word);
            return;
        }
    }
}

fn handle_input(input: &str) {
    match input {
        ".exit" => std::process::exit(0),
        ".help" => println!("Help message"),
        _ => println!("Unknown command: {}", input),
    }
}
