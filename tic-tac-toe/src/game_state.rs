use quicksilver::{
    graphics::Color,
    lifecycle::{State, Window},
    Result as QSResult,
};

use lib_tac_toe::Game;

mod assets;
pub mod grid;

use assets::*;
use grid::*;

const BG_COLOR: &str = "f2eecb";
const GRID_PADDING: f32 = 46.5;

pub struct TicTacToe {
    pub grid: Grid,
    assets: GameAssets,
    game: Game,
}

impl State for TicTacToe {
    fn new() -> QSResult<Self> {
        let assets = GameAssets::new()?;
        let grid = GridBuilder::new()
            .with_padding(GRID_PADDING)
            .with_line_size(assets.line_size())
            .with_reference_point((0.0, 0.0))
            .build();
        let game = Game::new();

        Ok(Self { assets, grid, game })
    }

    fn draw(&mut self, window: &mut Window) -> QSResult<()> {
        window.clear(Color::from_hex(BG_COLOR))?;

        self.grid.draw(window, &self.assets);

        Ok(())
    }
}
