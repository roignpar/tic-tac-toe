use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::Background::Img,
    lifecycle::Window,
};

use lib_tac_toe::CellCoord;

use super::assets::*;
use crate::calc::*;

type GridCells = [[Cell; 3]; 3];

#[derive(Default)]
pub struct GridBuilder {
    reference: Vector,
    padding: f32,
    line_size: f32,

    // helper params that cannot be set, but are
    // calculated from the other params
    padded_reference: Vector,
    bottom_right: Vector,
    mid: Vector,
    cell_len: f32,
}

/// Given a reference point (top-left), wanted padding and the length
/// of a grid line will construct the entire grid model.
impl GridBuilder {
    pub fn new() -> GridBuilder {
        GridBuilder::default()
    }

    pub fn with_reference_point(self, point: impl Into<Vector>) -> Self {
        Self {
            reference: point.into(),
            ..self
        }
    }

    pub fn with_padding(self, padding: f32) -> Self {
        Self { padding, ..self }
    }

    pub fn with_line_size(self, line_size: f32) -> Self {
        Self { line_size, ..self }
    }

    pub fn build(mut self) -> Grid {
        self.calc_helpers();

        Grid {
            line_mids: self.build_line_mids(),
            cells: self.build_cells(),
            total_width: self.line_size + 2.0 * self.padding,
            total_height: self.line_size + 2.0 * self.padding,
            top_left: self.padded_reference,
            bottom_right: self.bottom_right,
        }
    }

    fn calc_helpers(&mut self) {
        self.padded_reference = (
            self.reference.x + self.padding,
            self.reference.y + self.padding,
        )
            .into();
        self.bottom_right = (
            self.padded_reference.x + self.line_size,
            self.padded_reference.y + self.line_size,
        )
            .into();
        self.mid = midpoint(self.padded_reference, self.bottom_right);
        self.cell_len = self.line_size / 3.0;
    }

    fn build_line_mids(&self) -> LineMids {
        LineMids {
            vertical: VertLineMids {
                left: (self.cell_len + self.padded_reference.x, self.mid.y).into(),
                right: (self.cell_len * 2.0 + self.padded_reference.x, self.mid.y).into(),
            },
            horizontal: HorizLineMids {
                top: (self.mid.x, self.cell_len + self.padded_reference.y).into(),
                bottom: (self.mid.x, self.cell_len * 2.0 + self.padded_reference.y).into(),
            },
        }
    }

    fn build_cells(&self) -> GridCells {
        let mut cells = [[Cell::default(); 3]; 3];

        for (i, column) in cells.iter_mut().enumerate() {
            for (j, cell) in column.iter_mut().enumerate() {
                cell.top_left = (
                    i as f32 * self.cell_len + self.padded_reference.x,
                    j as f32 * self.cell_len + self.padded_reference.y,
                )
                    .into();
                cell.bottom_right = (
                    (i as f32 + 1.0) * self.cell_len + self.padded_reference.x,
                    (j as f32 + 1.0) * self.cell_len + self.padded_reference.y,
                )
                    .into();
                cell.mid = midpoint(cell.top_left, cell.bottom_right);
            }
        }

        cells
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    top_left: Vector,
    bottom_right: Vector,
    pub total_width: f32,
    pub total_height: f32,
    pub line_mids: LineMids,
    pub cells: GridCells,
}

#[derive(Debug, Default)]
pub struct LineMids {
    pub vertical: VertLineMids,
    pub horizontal: HorizLineMids,
}

#[derive(Debug, Default)]
pub struct VertLineMids {
    pub left: Vector,
    pub right: Vector,
}

#[derive(Debug, Default)]
pub struct HorizLineMids {
    pub top: Vector,
    pub bottom: Vector,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub top_left: Vector,
    pub bottom_right: Vector,
    pub mid: Vector,
}

impl Cell {
    fn contains(&self, v: Vector) -> bool {
        inside_rectangle(self.top_left, self.bottom_right, v)
    }
}

impl Grid {
    pub fn draw(&self, window: &mut Window, assets: &GameAssets) {
        for mid in self.v_line_mids().iter() {
            window.draw(&assets.line.area().with_center(*mid), Img(&assets.line));
        }

        for mid in self.h_line_mids().iter() {
            window.draw_ex(
                &assets.line.area().with_center(*mid),
                Img(&assets.line),
                Transform::rotate(90),
                0,
            );
        }
    }

    pub fn cell_containing(&self, v: Vector) -> Option<(CellCoord, &Cell)> {
        for (i, column) in self.cells.iter().enumerate() {
            for (j, cell) in column.iter().enumerate() {
                if cell.contains(v) {
                    return Some(((i, j), cell));
                }
            }
        }

        None
    }

    pub fn inside_grid(&self, v: Vector) -> bool {
        inside_rectangle(self.top_left, self.bottom_right, v)
    }

    fn h_line_mids(&self) -> [Vector; 2] {
        [
            self.line_mids.horizontal.top,
            self.line_mids.horizontal.bottom,
        ]
    }

    fn v_line_mids(&self) -> [Vector; 2] {
        [self.line_mids.vertical.left, self.line_mids.vertical.right]
    }
}

fn inside_rectangle(top_left: Vector, bottom_right: Vector, point: Vector) -> bool {
    point.x > top_left.x
        && point.y > top_left.y
        && point.x < bottom_right.x
        && point.y < bottom_right.y
}
