use macroquad::{prelude::Texture2D};

use crate::court::Glyph;

pub struct Icons {
    pub lparen: Texture2D,
    pub rparen: Texture2D,

    pub bat: Texture2D,
    pub bat_potion: Texture2D,
    pub feather: Texture2D,
    pub raptor: Texture2D,
    pub robe: Texture2D,
    pub shield_1: Texture2D,
}

pub fn load() -> Icons {
    Icons { 
        lparen: Texture2D::from_file_with_format(include_bytes!("lparen.png"), None),
        rparen: Texture2D::from_file_with_format(include_bytes!("rparen.png"), None),

        bat: Texture2D::from_file_with_format(include_bytes!("bat.png"), None),
        bat_potion: Texture2D::from_file_with_format(include_bytes!("bat_potion.png"), None),
        feather: Texture2D::from_file_with_format(include_bytes!("feather.png"), None) ,
        raptor: Texture2D::from_file_with_format(include_bytes!("raptor.png"), None) ,
        robe: Texture2D::from_file_with_format(include_bytes!("robe.png"), None) ,
        shield_1: Texture2D::from_file_with_format(include_bytes!("shield_1.png"), None) ,
    }
}
impl Icons {
    pub(crate) fn pick(&self, glyph: &Glyph) -> Texture2D {
        match glyph {
            Glyph::Single(atom) => match atom {
                crate::court::Atom::K => self.bat_potion,
                crate::court::Atom::S => self.feather,
                crate::court::Atom::I => self.raptor,
                crate::court::Atom::Y => self.bat,
            }
            Glyph::LParen => self.lparen,
            Glyph::RParen => self.rparen,
            Glyph::Unknown => self.robe,
        }
    }
}