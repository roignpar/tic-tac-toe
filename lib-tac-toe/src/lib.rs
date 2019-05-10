use failure::{bail, ensure, Error};

pub mod errors;

use errors::*;

const ROW_SIZE: usize = 3;
const MAX_INDEX: usize = ROW_SIZE - 1;
const MAX_TURNS: u8 = 5;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum XorZ {
    X,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Marked(XorZ),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    Draw,
    Win(XorZ),
}

pub type CellCoord = (usize, usize);
pub type BoardState = [[CellState; ROW_SIZE]; ROW_SIZE];

pub struct Game {
    turn_number: u8,
    /// who's turn is it?
    turn_of: XorZ,
    state: BoardState,
    outcome: Option<Outcome>,
}

impl Game {
    /// Creates a new game with the default starting state.
    pub fn new() -> Self {
        Game {
            turn_number: 1,
            turn_of: XorZ::X,
            state: [[CellState::Empty; ROW_SIZE]; ROW_SIZE],
            outcome: None,
        }
    }

    /// Places the next X or 0 on the board.
    pub fn mark(&mut self, x: usize, y: usize) -> MarkResult {
        if self.outcome.is_some() {
            bail!(MarkError::GameEnded);
        }

        Self::check_index_bounds(x, y)?;

        let cell = &mut self.state[x][y];

        if let CellState::Marked(_) = cell {
            bail!(MarkError::CellMarked);
        };

        *cell = CellState::Marked(self.turn_of);

        let outcome = self.check_outcome(x, y);
        if outcome.is_some() {
            return Ok(outcome);
        }

        self.advance_turn();

        Ok(None)
    }

    pub fn turn(&self) -> XorZ {
        self.turn_of
    }

    pub fn board_state(&self) -> &BoardState {
        &self.state
    }

    fn check_outcome(&mut self, last_x: usize, last_y: usize) -> Option<Outcome> {
        // there cannot be a winner before turn 3
        if (self.turn_number as usize) < ROW_SIZE {
            return None;
        }

        let won =
            self.won_v(last_x, last_y) || self.won_h(last_x, last_y) || self.won_d(last_x, last_y);

        if won {
            self.outcome = Some(Outcome::Win(self.turn_of));
            return self.outcome;
        }

        if self.turn_number == MAX_TURNS {
            self.outcome = Some(Outcome::Draw);
            return self.outcome;
        }

        None
    }

    /// Vertical.
    fn won_v(&self, x: usize, y: usize) -> bool {
        let cell1 = self.state[x][y];

        let (y2, y3) = Self::other_two_indexes(y);

        let cell2 = self.state[x][y2];
        let cell3 = self.state[x][y3];

        Self::marked_same(cell1, cell2, cell3)
    }

    /// Horizontal
    fn won_h(&self, x: usize, y: usize) -> bool {
        let cell1 = self.state[x][y];

        let (x2, x3) = Self::other_two_indexes(x);

        let cell2 = self.state[x2][y];
        let cell3 = self.state[x3][y];

        Self::marked_same(cell1, cell2, cell3)
    }

    /// Diagonal
    fn won_d(&self, x: usize, y: usize) -> bool {
        match (x, y) {
            (1, 1) => self.won_d_left() || self.won_d_right(),
            (0, 0) | (2, 2) => self.won_d_left(),
            (2, 0) | (0, 2) => self.won_d_right(),
            _ => false,
        }
    }

    /// Diagonal 0 0, 1 1, 2 2
    fn won_d_left(&self) -> bool {
        Self::marked_same(self.state[0][0], self.state[1][1], self.state[2][2])
    }

    /// Diagonal 2 0, 1 1, 0 2
    fn won_d_right(&self) -> bool {
        Self::marked_same(self.state[2][0], self.state[1][1], self.state[0][2])
    }

    fn other_two_indexes(i: usize) -> CellCoord {
        match i {
            0 => (1, 2),
            1 => (0, 2),
            2 => (0, 1),
            _ => unreachable!(),
        }
    }

    fn marked_same(cell1: CellState, cell2: CellState, cell3: CellState) -> bool {
        cell1 == cell2 && cell2 == cell3
    }

    fn advance_turn(&mut self) {
        use XorZ::*;

        match self.turn_of {
            X => self.turn_of = Z,
            Z => {
                self.turn_of = X;
                self.turn_number += 1;
            }
        }
    }

    fn check_index_bounds(x: usize, y: usize) -> Result<(), Error> {
        ensure!(x <= MAX_INDEX, MarkError::new_oob(x));
        ensure!(y <= MAX_INDEX, MarkError::new_oob(y));
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Outcome::*;
    use XorZ::*;

    // TODO: find a way to check that the returned errors have the correct types

    #[test]
    fn marking_out_of_bounds() {
        let mut g = Game::new();

        let cases = [(0, 3), (3, 0)];

        for (x, y) in &cases {
            assert!(g.mark(*x, *y).is_err());
        }
    }

    #[test]
    fn marking_marked() {
        let mut g = Game::new();

        assert!(g.mark(0, 0).is_ok());
        assert!(g.mark(0, 0).is_err());
    }

    #[test]
    fn horizontal_win() {
        // as X
        let mut g = horizontal_game_start();

        assert_winner(g.mark(2, 0), X);

        // as 0
        g = horizontal_game_start();

        g.mark(0, 2).unwrap();

        assert_winner(g.mark(2, 1), Z);
    }

    fn horizontal_game_start() -> Game {
        game_with_markings(&[(0, 0), (0, 1), (1, 0), (1, 1)])
    }

    #[test]
    fn vertical_win() {
        // as X
        let mut g = vertical_game_start();

        assert_winner(g.mark(0, 2), X);

        // as 0
        g = vertical_game_start();

        g.mark(2, 0).unwrap();

        assert_winner(g.mark(1, 2), Z);
    }

    fn vertical_game_start() -> Game {
        game_with_markings(&[(0, 0), (1, 0), (0, 1), (1, 1)])
    }

    #[test]
    fn left_diagonal_win() {
        let mut g = game_with_markings(&[(0, 0), (0, 1), (1, 1), (0, 2)]);

        assert_winner(g.mark(2, 2), X);

        g = game_with_markings(&[(1, 0), (0, 0), (1, 2), (1, 1), (0, 1)]);

        assert_winner(g.mark(2, 2), Z);
    }

    #[test]
    fn righ_diagonal_win() {
        let mut g = game_with_markings(&[(2, 0), (0, 0), (1, 1), (0, 1)]);

        assert_winner(g.mark(0, 2), X);

        g = game_with_markings(&[(0, 0), (2, 0), (1, 0), (1, 1), (0, 1)]);

        assert_winner(g.mark(0, 2), Z);
    }

    #[test]
    fn marking_finished() {
        let mut g = game_with_markings(&[(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)]);

        assert!(g.mark(2, 2).is_err());
    }

    #[test]
    fn draw() {
        let mut g = game_with_markings(&[
            (0, 0),
            (1, 1),
            (0, 1),
            (0, 2),
            (2, 0),
            (1, 0),
            (1, 2),
            (2, 2),
        ]);

        assert_outcome(g.mark(2, 1), Draw);
    }

    #[test]
    fn turns() {
        let mut g = Game::new();

        assert_turn(&g, 1, X);

        g.mark(0, 0).unwrap();

        assert_turn(&g, 1, Z);

        g.mark(1, 1).unwrap();

        assert_turn(&g, 2, X);

        g.mark(0, 1).unwrap();

        assert_turn(&g, 2, Z);

        g.mark(0, 2).unwrap();

        assert_turn(&g, 3, X);

        g.mark(2, 0).unwrap();

        assert_turn(&g, 3, Z);

        g.mark(1, 0).unwrap();

        assert_turn(&g, 4, X);

        g.mark(1, 2).unwrap();

        assert_turn(&g, 4, Z);

        g.mark(2, 2).unwrap();

        assert_turn(&g, 5, X);

        assert_outcome(g.mark(2, 1), Draw);

        // last move should not change the turn because
        // there are no more turns after it
        assert_turn(&g, 5, X);
    }

    fn game_with_markings(m: &[CellCoord]) -> Game {
        let mut g = Game::new();

        for (x, y) in m {
            g.mark(*x, *y).unwrap();
        }

        g
    }

    fn assert_winner(r: MarkResult, xz: XorZ) {
        assert_outcome(r, Win(xz));
    }

    fn assert_outcome(r: MarkResult, o: Outcome) {
        match r.unwrap().unwrap() {
            outcome if outcome == o => (),
            _ => panic!("{:?} should have been the outcome!", o),
        }
    }

    fn assert_turn(g: &Game, number: u8, of: XorZ) {
        assert_eq!(g.turn_number, number);
        assert_eq!(g.turn_of, of);
    }
}
