use quicksilver::{
    geom::{Shape, Transform},
    graphics::Background::Img,
    lifecycle::Window,
};

use super::assets::*;
use crate::calc::*;

type GridCells = [[Cell; 3]; 3];

#[derive(Default)]
pub struct GridBuilder {
    reference: RawPoint,
    padding: f32,
    line_size: f32,

    // helper params that cannot be set, but are
    // calculated from the other params
    padded_reference: RawPoint,
    bottom_right: RawPoint,
    mid: RawPoint,
    cell_len: f32,
}

/// Given a reference point (top-left), wanted padding and the length
/// of a grid line will construct the entire grid model.
impl GridBuilder {
    pub fn new() -> GridBuilder {
        GridBuilder::default()
    }

    pub fn with_reference_point(self, point: RawPoint) -> Self {
        Self {
            reference: point,
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
        }
    }

    fn calc_helpers(&mut self) {
        self.padded_reference = (
            self.reference.0 + self.padding,
            self.reference.1 + self.padding,
        );
        self.bottom_right = (
            self.padded_reference.0 + self.line_size,
            self.padded_reference.1 + self.line_size,
        );
        self.mid = midpoint(self.padded_reference, self.bottom_right);
        self.cell_len = self.line_size / 3.0;
    }

    fn build_line_mids(&self) -> LineMids {
        LineMids {
            vertical: VertLineMids {
                left: (self.cell_len + self.padded_reference.0, self.mid.1),
                right: (self.cell_len * 2.0 + self.padded_reference.0, self.mid.1),
            },
            horizontal: HorizLineMids {
                top: (self.mid.0, self.cell_len + self.padded_reference.1),
                bottom: (self.mid.0, self.cell_len * 2.0 + self.padded_reference.1),
            },
        }
    }

    fn build_cells(&self) -> GridCells {
        let mut cells = [[Cell::default(); 3]; 3];

        for (i, column) in cells.iter_mut().enumerate() {
            for (j, cell) in column.iter_mut().enumerate() {
                cell.top_left = (
                    i as f32 * self.cell_len + self.padded_reference.0,
                    j as f32 * self.cell_len + self.padded_reference.1,
                );
                cell.bottom_right = (
                    (i as f32 + 1.0) * self.cell_len + self.padded_reference.0,
                    (j as f32 + 1.0) * self.cell_len + self.padded_reference.1,
                );
                cell.mid = midpoint(cell.top_left, cell.bottom_right);
            }
        }

        cells
    }
}

#[derive(Debug, Default)]
pub struct Grid {
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
    pub left: RawPoint,
    pub right: RawPoint,
}

#[derive(Debug, Default)]
pub struct HorizLineMids {
    pub top: RawPoint,
    pub bottom: RawPoint,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub top_left: RawPoint,
    pub bottom_right: RawPoint,
    pub mid: RawPoint,
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

    fn h_line_mids(&self) -> [RawPoint; 2] {
        [
            self.line_mids.horizontal.top,
            self.line_mids.horizontal.bottom,
        ]
    }

    fn v_line_mids(&self) -> [RawPoint; 2] {
        [self.line_mids.vertical.left, self.line_mids.vertical.right]
    }
}
