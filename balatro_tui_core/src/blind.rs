//! This module contains the implementation of different blind types and their
//! variants.
//!
//! The [`Blind`] enum is the entrypoint, data carrier and defines property
//! access methods.

use std::num::NonZeroUsize;

use strum::{
    Display as EnumDisplay, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr,
    VariantArray,
};

use crate::{
    enum_property_ext::EnumPropertyExt,
    error::{ArithmeticError, ScorerError, StrumError},
};

/// Blind type can be either small blind, big blind or boss blind.
///
/// Within boss blind, the boss can be one of the valid [`Bosses`].
///
/// A blind type has associated `score_multiplier`, `color` and `reward`
/// properties that can be fetched using [`EnumProperty::get_str()`].
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    EnumDisplay,
    EnumCount,
    EnumProperty,
    Eq,
    Hash,
    IntoStaticStr,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[repr(usize)]
pub enum Blind {
    /// The default blind, small blind is the first blind faced in the run. It
    /// is an optional blind and can be skipped.
    #[default]
    #[strum(
        serialize = "Small Blind",
        props(score_multiplier = "2", color = "blue", reward = "3")
    )]
    Small,
    /// The big blind is the second blind faced in the run. It is an optional
    /// blind and can be skipped.
    #[strum(
        serialize = "Big Blind",
        props(score_multiplier = "3", color = "green", reward = "4")
    )]
    Big,
    /// The boss blind is required to be played. Associated is one of the
    /// [`Bosses`] that has a special power.
    #[strum(
        serialize = "Boss Blind",
        props(score_multiplier = "4", color = "red", reward = "5")
    )]
    Boss(Bosses),
}

/// Bosses are different blinds that can be randomly show up during a run as
/// boss blind. Each boss has a unique associated power that plays out during
/// the boss blind round.
#[derive(
    Clone,
    Copy,
    Debug,
    EnumDisplay,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    IntoStaticStr,
    Ord,
    PartialEq,
    PartialOrd,
    VariantArray,
)]
#[strum(prefix = "The ")]
pub enum Bosses {
    /// Discards 2 random cards from your hand, after each hand played
    Hook,
    /// Playing your most played hand this run sets money to $0
    Ox,
    /// First hand is drawn face down
    House,
    /// Extra large blind (2x base target)
    Wall,
    /// 1 in 7 cards get drawn face-down throughout the round
    Wheel,
    ///  Decreases the level of Hand you play by 1 (hand levels can go to Level
    /// 1, and are permanently reduced before scoring)
    Arm,
    /// All Club cards are debuffed
    Club,
    /// Cards are drawn face down after each hand played
    Fish,
    /// Must play 5 cards (they do not need to be scoring)
    Psychic,
    /// All Spade cards are debuffed
    Goad,
    /// Start with 0 discards
    Water,
    /// All Diamond cards are debuffed
    Window,
    /// -1 Hand Size
    Manacle,
    /// Every hand played this round must be of a different type and not
    /// previously played this round
    Eye,
    /// Only one hand type can be played this round
    Mouth,
    /// All face cards are debuffed
    Plant,
    /// After playing a hand or discarding cards, you always draw 3 cards (hand
    /// size is ignored)
    Serpent,
    /// Cards played previously this Ante (during Small and Big Blinds) are
    /// debuffed
    Pillar,
    /// Play only 1 hand (0.5x base target)
    Needle,
    /// All Heart cards are debuffed
    Head,
    /// Lose $1 per card played
    Tooth,
    /// The base Chips and Multiplier for playing a poker hand are halved this
    /// round
    Flint,
    /// All face cards are drawn face down
    Mark,
}

const BLIND_BASE_AMOUNTS: [usize; 8] = [3, 8, 20, 50, 110, 200, 350, 500];

impl Blind {
    /// Returns the target score required to cross the round with this blind.
    #[inline]
    pub fn get_target_score(&self, ante: NonZeroUsize) -> Result<usize, ScorerError> {
        if ante.get() >= BLIND_BASE_AMOUNTS.len() {
            return Err(ScorerError::AnteExceeded(ante.get()));
        }

        let blind_multiple = self.get_int_property("score_multiplier")?;

        let chips_multiplier: usize = 25;

        let boss_blind_multiplier = if let &Self::Boss(boss) = self {
            if boss == Bosses::Wall {
                4
            } else if boss == Bosses::Needle {
                1
            } else {
                2
            }
        } else {
            2
        };

        Ok(chips_multiplier
            .checked_mul(blind_multiple)
            .ok_or(ArithmeticError::Overflow("multiplication"))?
            .checked_mul(boss_blind_multiplier)
            .ok_or(ArithmeticError::Overflow("multiplication"))?
            .checked_mul(
                *BLIND_BASE_AMOUNTS
                    .get(
                        ante.get()
                            .checked_sub(1)
                            .ok_or(ArithmeticError::Overflow("subtraction"))?,
                    )
                    .ok_or_else(|| ScorerError::AnteExceeded(ante.get()))?,
            )
            .ok_or(ArithmeticError::Overflow("multiplication"))?)
    }

    /// Returns color used to represent the blind.
    #[inline]
    pub fn get_color(&self) -> Result<&str, StrumError> {
        self.get_property("color")
    }

    /// Returns the reward obtained after defeating the blind.
    #[inline]
    pub fn get_reward(&self) -> Result<usize, StrumError> {
        self.get_int_property("reward")
    }
}
