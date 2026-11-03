/// Given a collection of socks, that have an orientation and color
/// find how many of them can be paired based on orientation only,
///                              paired based on orientation and color,
///                              paired based on orientation and a parent color,
/// think what needs to be done if size were to be taken into account
use core::cmp::{Ord, min};
use core::convert::Into;
use core::iter::Iterator;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
    Teal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Orientation {
    Left,
    Right,
}
impl From<Orientation> for (u32, u32) {
    fn from(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Left => (1, 0),
            Orientation::Right => (0, 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Sock {
    pub orientation: Orientation,
    pub color: Color,
    // TODO: what if we wanted to add size as well?
}

impl Sock {
    pub fn as_tuple(&self) -> (Color, (u32, u32)) {
        (self.color, self.orientation.into())
    }

    // INFO: I think this was never asked
    pub fn as_grouped_tuple(&self) -> (Color, (u32, u32)) {
        match self.color {
            Color::Red | Color::Blue => (Color::Red, self.orientation.into()),
            Color::Green | Color::Teal => (Color::Green, self.orientation.into()),
        }
    }
}

pub fn by_orientation(socks: &[Sock]) -> u32 {
    let count = socks
        .iter()
        .map(|sock| sock.orientation.into())
        .fold((0, 0), |acc, (left, right)| (acc.0 + left, acc.1 + right));
    min(count.0, count.1)
}

pub fn by_orientation_and_color(socks: &[Sock]) -> u32 {
    let mut counts = HashMap::new();
    for (color, (left, right)) in socks.iter().map(Sock::as_tuple) {
        let entry = counts.entry(color).or_insert((0, 0));
        *entry = (entry.0 + left, entry.1 + right)
    }
    counts
        .iter()
        .map(|(_, (left, right))| *left.min(right))
        .sum()
}

pub fn by_orientation_and_color_group(socks: &[Sock]) -> u32 {
    let mut counts = HashMap::new();
    for (color, (left, right)) in socks.iter().map(Sock::as_grouped_tuple) {
        let entry = counts.entry(color).or_insert((0, 0));
        *entry = (entry.0 + left, entry.1 + right)
    }
    counts
        .iter()
        .map(|(_, (left, right))| *left.min(right))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! sock {
        ($orientation:ident, $color: ident) => {
            Sock {
                orientation: Orientation::$orientation,
                color: Color::$color,
            }
        };
    }

    #[test]
    fn orientation_match_basic() {
        let socks = vec![
            sock!(Left, Red),
            sock!(Right, Green),
            sock!(Left, Blue),
            sock!(Right, Teal),
        ];
        assert_eq!(by_orientation(&socks), 2);
    }

    #[test]
    fn orientation_and_color_matches() {
        let socks = vec![
            sock!(Left, Red),
            sock!(Right, Red),
            sock!(Left, Blue),
            sock!(Right, Teal),
        ];
        assert_eq!(by_orientation_and_color(&socks), 1);
    }
}
