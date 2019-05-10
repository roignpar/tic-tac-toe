use quicksilver::{
    geom::Shape,
    graphics::{Background::Img, Color, Image},
    lifecycle::{State, Window},
    Result as QSResult,
};

use lib_tac_toe::{CellState, Game, XorZ};

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

        self.draw_game_state(window);

        Ok(())
    }
}

impl TicTacToe {
    fn draw_game_state(&self, window: &mut Window) {
        let board = self.game.board_state();

        for (i, column) in board.iter().enumerate() {
            for (j, cell) in column.iter().enumerate() {
                if let CellState::Marked(mark) = cell {
                    let img = self.x_z_image(*mark);
                    let center = self.grid.cells[i][j].mid;

                    window.draw(&img.area().with_center(center), Img(&img));
                }
            }
        }
    }

    fn x_z_image(&self, xz: XorZ) -> &Image {
        use XorZ::*;

        match xz {
            X => &self.assets.x,
            Z => &self.assets.z,
        }
    }
}
