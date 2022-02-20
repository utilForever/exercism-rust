use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

use std::cmp::Ord;
use std::fmt::Display;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, IntoEnumIterator, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => color.to_string(),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut ret: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    ret.sort();
    ret
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Ord for ResistorColor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        color_to_value(*self).cmp(&color_to_value(*other))
    }
}

impl PartialOrd for ResistorColor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(color_to_value(*self).cmp(&color_to_value(*other)))
    }
}

impl PartialEq for ResistorColor {
    fn eq(&self, other: &Self) -> bool {
        color_to_value(*self) == color_to_value(*other)
    }
}
