mod handler;

use handler::input::user_input;

fn main() {
    loop {
        user_input();
    }
}
