use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{
        Background::{Blended, Img},
        Color, Image,
    },
    input::{Mouse, MouseButton},
    lifecycle::{State, Window},
    Result as QSResult,
};

use lib_tac_toe::{CellState, Game, Outcome, WinLine, XorZ};

mod assets;
mod buttons;
mod commands;
pub mod grid;

use assets::*;
use grid::*;

const BG_COLOR: &str = "f2eecb";
const GRID_PADDING: f32 = 46.5;
const MARK_SHADOW_ALPHA: f32 = 0.09;

// win line angles; a bit skewed
const HWL_ANGLE: i16 = 88;
const VWL_ANGLE: i16 = -2;
const DLWL_ANGLE: i16 = -42;
const DRWL_ANGLE: i16 = 47;

const WL_SCALE: (f32, f32) = (1.0, 0.94);
const DWL_SCALE: (f32, f32) = (1.0, 1.3);

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

        self.draw_mark_shadow(window);

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> QSResult<()> {
        self.handle_mouse(window.mouse());

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

                    window.draw(&img.area().with_center(center), Img(img));
                }
            }
        }

        self.draw_win_line(window);
    }

    fn draw_win_line(&self, window: &mut Window) {
        if let Some(Outcome::Win(_, wl)) = self.game.get_outcome() {
            let (center, angle) = self.win_line_center_angle(wl);

            window.draw_ex(
                &self.assets.line.area().with_center(center),
                Img(&self.assets.line),
                Transform::rotate(angle) * Transform::scale(self.win_line_scale(wl)),
                0,
            );
        }
    }

    fn draw_mark_shadow(&self, window: &mut Window) {
        if self.game.ended() {
            return;
        }

        if let Some((coord, cell)) = self.grid.cell_containing(window.mouse().pos()) {
            if !self.game.is_marked(coord.0, coord.1) {
                let img = self.x_z_image(self.game.turn());
                let color = Color::from_rgba(0, 0, 0, MARK_SHADOW_ALPHA);

                window.draw(&img.area().with_center(cell.mid), Blended(img, color));
            }
        }
    }

    fn win_line_center_angle(&self, line: WinLine) -> (Vector, i16) {
        use WinLine::*;

        let cells = &self.grid.cells;

        match line {
            HTop => (cells[1][0].mid, HWL_ANGLE),
            HMid => (cells[1][1].mid, HWL_ANGLE),
            HBottom => (cells[1][2].mid, HWL_ANGLE),

            VLeft => (cells[0][1].mid, VWL_ANGLE),
            VMid => (cells[1][1].mid, VWL_ANGLE),
            VRight => (cells[2][1].mid, VWL_ANGLE),

            DLeft => (cells[1][1].mid, DLWL_ANGLE),
            DRight => (cells[1][1].mid, DRWL_ANGLE),
        }
    }

    fn win_line_scale(&self, line: WinLine) -> impl Into<Vector> {
        use WinLine::*;

        match line {
            DRight | DLeft => DWL_SCALE,
            _ => WL_SCALE,
        }
    }

    fn x_z_image(&self, xz: XorZ) -> &Image {
        use XorZ::*;

        match xz {
            X => &self.assets.x,
            Z => &self.assets.z,
        }
    }

    fn handle_mouse(&mut self, mouse: Mouse) {
        let position = mouse.pos();

        if mouse[MouseButton::Left].is_down() {
            if let Some((coord, _)) = self.grid.cell_containing(position) {
                let _ = self.game.mark(coord.0, coord.1);
            }
        }
    }
}
