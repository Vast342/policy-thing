/*
    Anura
    Copyright (C) 2024 Joseph Pasfield

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::fmt;

pub static SQUARE_NAMES: [&str; 64] = [
    "a1","b1","c1","d1","e1","f1","g1","h1",
    "a2","b2","c2","d2","e2","f2","g2","h2",
    "a3","b3","c3","d3","e3","f3","g3","h3",
    "a4","b4","c4","d4","e4","f4","g4","h4",
    "a5","b5","c5","d5","e5","f5","g5","h5",
    "a6","b6","c6","d6","e6","f6","g6","h6",
    "a7","b7","c7","d7","e7","f7","g7","h7",
    "a8","b8","c8","d8","e8","f8","g8","h8",
];

#[derive(Debug, Copy, Clone, Default)]
pub struct Move(pub u16);

#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
pub enum Flag {
    Normal,
    WKCastle,
    WQCastle,
    BKCastle,
    BQCastle,
    EnPassant,
    DoublePush,
    KnightPromo,
    BishopPromo,
    RookPromo,
    QueenPromo,
}

impl Flag {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::WKCastle,
            2 => Self::WQCastle,
            3 => Self::BKCastle,
            4 => Self::BQCastle,
            5 => Self::EnPassant,
            6 => Self::DoublePush,
            7 => Self::KnightPromo,
            8 => Self::BishopPromo,
            9 => Self::RookPromo,
            10 => Self::QueenPromo,
            _ => panic!("invalid flag {value}"),
        }
    }
}

impl Move {
    #[must_use] pub const fn new_unchecked(from: u8, to: u8, flag: u8) -> Self {
        //debug_assert!(from <= 63, "invalid from square {from}");
        //debug_assert!(to <= 63, "invalid to square {to}");
        Self(((flag as u16) << 12) | ((to as u16) << 6) | from as u16)
    }
    pub const NULL_MOVE: Self = Self::new_unchecked(0, 0, 0);
    #[must_use] pub const fn from(&self) -> u8 {
        (self.0 & 0b11_1111) as u8
    }
    #[must_use] pub const fn to(&self) -> u8 {
        ((self.0 >> 6) & 0b11_1111) as u8
    }
    #[must_use] pub fn flag(&self) -> Flag {
        Flag::from_u8((self.0 >> 12) as u8)
    }
    pub fn to_other_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut c: String = SQUARE_NAMES[self.from() as usize].to_owned();
        c += SQUARE_NAMES[self.to() as usize];
        c += match self.flag() {
            Flag::KnightPromo => "n",
            Flag::BishopPromo => "b",
            Flag::RookPromo => "r",
            Flag::QueenPromo => "q",
            _ => "",
        };
        write!(f, "{c}")
    }
}