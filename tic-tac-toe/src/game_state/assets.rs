use quicksilver::{
    graphics::{Font, Image},
    Result as QSResult,
};

pub struct GameAssets {
    pub line: Image,
    pub x: Image,
    pub z: Image,
    pub font: Font,
}

impl GameAssets {
    pub fn new() -> QSResult<Self> {
        Ok(Self {
            line: Image::from_bytes(include_bytes!("../../../static/line.png"))?,
            x: Image::from_bytes(include_bytes!("../../../static/x.png"))?,
            z: Image::from_bytes(include_bytes!("../../../static/z.png"))?,
            font: Font::from_bytes(
                include_bytes!("../../../static/font/DeliusUnicase-Bold.ttf").to_vec(),
            )?,
        })
    }

    pub fn line_size(&self) -> f32 {
        let area = self.line.area();

        area.height().max(area.width())
    }
}
