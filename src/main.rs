use term::prompt_new_game;

mod game;
mod term;

fn main() {
    let game = prompt_new_game();
    if let Err(error) = game {
        println!("{:?}", error);
    }
}
