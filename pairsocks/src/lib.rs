//! Given a collection of socks, each with an orientation and a color, count how
//! many pairs can be formed when matching on
//!   - orientation only,
//!   - orientation and color,
//!   - orientation and a parent color group.
//!
//! Every variant is the same algorithm over a different grouping key, so they
//! all delegate to [`pairs_by`]. Adding a dimension (size, say) means adding a
//! field and a key function, not another counting loop.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
    Teal,
}

impl Color {
    /// The parent color this shade belongs to.
    pub fn group(self) -> ColorGroup {
        match self {
            Self::Red | Self::Blue => ColorGroup::Red,
            Self::Green | Self::Teal => ColorGroup::Green,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorGroup {
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sock {
    pub orientation: Orientation,
    pub color: Color,
}

/// Left/right tally for one grouping key.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Tally {
    left: u32,
    right: u32,
}

impl Tally {
    fn push(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Left => self.left += 1,
            Orientation::Right => self.right += 1,
        }
    }

    fn pairs(&self) -> u32 {
        self.left.min(self.right)
    }
}

/// Counts pairs among `socks`, where two socks may pair only if they have
/// opposite orientations and `key` maps them to the same value.
pub fn pairs_by<'a, K, F>(socks: impl IntoIterator<Item = &'a Sock>, key: F) -> u32
where
    K: Eq + Hash,
    F: Fn(&Sock) -> K,
{
    let mut tallies: HashMap<K, Tally> = HashMap::new();
    for sock in socks {
        tallies.entry(key(sock)).or_default().push(sock.orientation);
    }
    tallies.values().map(Tally::pairs).sum()
}

pub fn by_orientation(socks: &[Sock]) -> u32 {
    pairs_by(socks, |_| ())
}

pub fn by_orientation_and_color(socks: &[Sock]) -> u32 {
    pairs_by(socks, |sock| sock.color)
}

pub fn by_orientation_and_color_group(socks: &[Sock]) -> u32 {
    pairs_by(socks, |sock| sock.color.group())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! socks {
        ($(($orientation:ident, $color:ident)),* $(,)?) => {
            [$(Sock { orientation: Orientation::$orientation, color: Color::$color }),*]
        };
    }

    #[test]
    fn orientation_match_basic() {
        let socks = socks![(Left, Red), (Right, Green), (Left, Blue), (Right, Teal)];
        assert_eq!(by_orientation(&socks), 2);
    }

    #[test]
    fn orientation_and_color_matches() {
        let socks = socks![(Left, Red), (Right, Red), (Left, Blue), (Right, Teal)];
        assert_eq!(by_orientation_and_color(&socks), 1);
    }

    #[test]
    fn color_group_pairs_across_shades() {
        let socks = socks![(Left, Red), (Right, Blue), (Left, Green), (Right, Teal)];
        // No exact-color pair exists, but each sock finds a partner in its group.
        assert_eq!(by_orientation_and_color(&socks), 0);
        assert_eq!(by_orientation_and_color_group(&socks), 2);
    }

    #[test]
    fn no_pairs_when_all_one_orientation() {
        let socks = socks![(Left, Red), (Left, Red), (Left, Teal)];
        assert_eq!(by_orientation(&socks), 0);
        assert_eq!(by_orientation_and_color(&socks), 0);
    }

    #[test]
    fn empty_input_has_no_pairs() {
        assert_eq!(by_orientation(&[]), 0);
        assert_eq!(by_orientation_and_color(&[]), 0);
        assert_eq!(by_orientation_and_color_group(&[]), 0);
    }
}
