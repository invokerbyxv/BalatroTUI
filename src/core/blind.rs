use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use ratatui::style::Color;
use strum::{Display as EnumDisplay, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr, VariantArray};

#[derive(Clone, Copy, Debug, Default, EnumDisplay, EnumCount, EnumProperty, Eq, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd)]
#[repr(usize)]
pub enum BlindType {
    #[default]
    #[strum(serialize = "Small Blind", props(score_multiplier = "2", color = "blue"))]
    Small,
    #[strum(serialize = "Big Blind", props(score_multiplier = "3", color = "green"))]
    Big,
    #[strum(serialize = "Boss Blind", props(score_multiplier = "4", color = "red"))]
    Boss(Bosses),
}

#[derive(Clone, Copy, Debug, EnumDisplay, EnumCount, EnumIter, EnumString, Eq, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd, VariantArray,)]
#[strum(prefix = "The ")]
pub enum Bosses {
    Hook,
    House,
    Wall,
    Wheel,
    Arm,
    Club,
    Fish,
    Psychic,
    Goad,
    Water,
    Window,
    Manacle,
}

const BLIND_BASE_AMOUNTS: [usize; 8] = [3, 8, 20, 50, 110, 200, 350, 500];

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Blind {
    pub blind_type: BlindType,
    pub target: usize,
}

impl Blind {
    #[inline]
    fn get_target_score(blind_type: BlindType, ante: usize) -> Result<usize, Box<dyn Error>> {
        if ante >= BLIND_BASE_AMOUNTS.len() {
            // TODO: Implement endless mode blind base score calculation.
            Err("Ante has crossed maximum computable ante. Need additional implementation.")?;
        }

        let blind_type_multiple = blind_type.get_int("score_multiplier").unwrap();
        let chips_multiplier = 50;

        Ok(chips_multiplier * blind_type_multiple * BLIND_BASE_AMOUNTS[ante - 1])
    }

    #[inline]
    pub fn new(blind_type: BlindType, ante: usize) -> Result<Blind, Box<dyn Error>> {
        Ok(Blind {
            blind_type,
            target: Self::get_target_score(blind_type, ante)?,
        })
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        Color::from_str(self.blind_type.get_str("color").unwrap()).unwrap()
    }
}

impl Display for Blind {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.blind_type)
    }
}
