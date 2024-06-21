use std::error::Error;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, Hash, PartialEq)]
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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Blind {
    pub blind_type: BlindType,
    pub target: usize,
}

impl Blind {
    #[inline]
    fn get_target_score(blind_type: BlindType, ante: usize) -> Result<usize, Box<dyn Error>> {
        let ante_geometric_ratio: usize = f32::powi(2.5, (ante as isize - 2).try_into()?).floor() as usize;
        let ante_nth_geometric_term = 8 * ante_geometric_ratio;
        let blind_type_multiple = blind_type as usize + 2;
        let chips_multiplier = 50;

        Ok(chips_multiplier * ante_nth_geometric_term * blind_type_multiple)
    }

    #[inline]
    pub fn new(blind_type: BlindType, ante: usize) -> Result<Blind, Box<dyn Error>> {
        Ok(Blind {
            blind_type,
            target: Self::get_target_score(blind_type, ante)?,
        })
    }
}
