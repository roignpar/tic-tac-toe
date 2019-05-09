use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
    Result as QSResult,
};

mod calc;
mod game_state;

use game_state::GameState;

const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 650;

fn main() -> QSResult<()> {
    let window_size = Vector::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    run::<GameState>(
        "Tic-tac-toe",
        window_size,
        Settings {
            // no resizing
            min_size: Some(window_size),
            max_size: Some(window_size),
            ..Settings::default()
        },
    );

    Ok(())
}
