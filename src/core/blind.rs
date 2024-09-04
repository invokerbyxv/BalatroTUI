use std::{error::Error, fmt::{Display, Formatter, Result as FmtResult}};

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum BlindType {
    SmallBlind = 0,
    BigBlind = 1,
    BossBlind = 2,
}

pub enum Bosses {
    TheHook,
    TheHouse,
    TheWall,
    TheWheel,
    TheArm,
    TheClub,
    TheFish,
    ThePsychic,
    TheGoad,
    TheWater,
    TheWindow,
    TheManacle,
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

        let blind_type_multiple = blind_type as usize + 2;
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
        match self.blind_type {
            BlindType::SmallBlind => Color::Blue,
            BlindType::BigBlind => Color::Green,
            BlindType::BossBlind => Color::Red,
        }
    }
}

impl Display for Blind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let text = match self.blind_type {
            BlindType::SmallBlind => "Small Blind",
            BlindType::BigBlind => "Big Blind",
            BlindType::BossBlind => "Boss Blind",
        };
        write!(f, "{}", text)
    }
}