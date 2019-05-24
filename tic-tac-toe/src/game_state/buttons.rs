use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background, Color, Font, FontStyle, Image},
    lifecycle::Window,
    Result as QSResult,
};

use super::commands::*;
use crate::calc::*;

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.4,
    a: 1.0,
};

const BTN_HEIGHT: f32 = 40.0;
const BTN_ALPHA: f32 = 0.72;
const BTN_HL_ALPHA: f32 = 1.0;
const BTN_COLOR: Color = Color {
    r: 1.0,
    g: 0.7,
    b: 0.1,
    a: BTN_ALPHA,
};
const BTN_HL_COLOR: Color = Color {
    a: BTN_HL_ALPHA,
    ..BTN_COLOR
};
const BTN_SPACING: f32 = 20.0;

#[derive(Debug)]
pub struct GameButtons {
    top_left: Vector,
    bottom_right: Vector,
    buttons: Vec<GameButton>,
}

impl GameButtons {
    pub fn new(top_left: Vector, bottom_right: Vector, padding: f32) -> Self {
        Self {
            buttons: Vec::new(),
            top_left: Vector::new(top_left.x + padding, top_left.y + padding),
            bottom_right: Vector::new(bottom_right.x - padding, bottom_right.y - padding),
        }
    }

    pub fn add_button(&mut self, mut button: GameButton) {
        button.top_left = Vector::new(
            self.top_left.x,
            self.top_left.y + self.buttons.len() as f32 * (BTN_HEIGHT + BTN_SPACING),
        );
        button.bottom_right = Vector::new(
            button.top_left.x + self.btn_width(),
            button.top_left.y + BTN_HEIGHT,
        );

        self.buttons.push(button);
    }

    pub fn draw(&self, window: &mut Window) {
        for btn in self.buttons.iter() {
            btn.draw(window);
        }
    }

    /// Get the command, if any, of the button
    /// at the given position.
    pub fn btn_command(&self, position: Vector) -> Option<Command> {
        if !self.contains(position) {
            return None;
        }

        for btn in self.buttons.iter() {
            if btn.contains(position) {
                return Some(btn.command);
            }
        }

        None
    }

    fn btn_width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    fn contains(&self, v: Vector) -> bool {
        inside_rectangle(self.top_left, self.bottom_right, v)
    }
}

#[derive(Debug)]
pub struct GameButton {
    rendered_text: Image,
    /// command given by pressing this button
    command: Command,
    top_left: Vector,
    bottom_right: Vector,
}

impl GameButton {
    pub fn new(font: &Font, text: &str, command: Command) -> QSResult<Self> {
        Ok(Self {
            rendered_text: font.render(text, &font_style())?,
            command,
            top_left: Vector::new(0.0, 0.0),
            bottom_right: Vector::new(0.0, 0.0),
        })
    }

    fn draw(&self, window: &mut Window) {
        let mid = midpoint(self.top_left, self.bottom_right);
        let rectangle = self.bg_rectangle();
        let mouse_pos = window.mouse().pos();

        window.draw(
            &rectangle.with_center(mid),
            Background::Col(self.bg_color(mouse_pos)),
        );
        window.draw(
            &self.rendered_text.area().with_center(mid),
            Background::Blended(&self.rendered_text, self.text_blend_color(mouse_pos)),
        );
    }

    fn bg_rectangle(&self) -> Rectangle {
        let size = Vector::new(
            self.bottom_right.x - self.top_left.x,
            self.bottom_right.y - self.top_left.y,
        );

        Rectangle::new(self.top_left, size)
    }

    fn contains(&self, v: Vector) -> bool {
        inside_rectangle(self.top_left, self.bottom_right, v)
    }

    fn bg_color(&self, mouse_pos: Vector) -> Color {
        if self.contains(mouse_pos) {
            BTN_HL_COLOR
        } else {
            BTN_COLOR
        }
    }

    fn text_blend_color(&self, mouse_pos: Vector) -> Color {
        let alpha = if self.contains(mouse_pos) {
            BTN_HL_ALPHA
        } else {
            BTN_ALPHA
        };

        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: alpha,
        }
    }
}

fn font_style() -> FontStyle {
    FontStyle::new(FONT_SIZE, FONT_COLOR)
}
