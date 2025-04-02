use std::{fmt::Display, str::FromStr};

use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub enum Shape {
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Triangle => write!(f, "Triangle"),
            Shape::Square => write!(f, "Square"),
            Shape::Pentagon => write!(f, "Pentagon"),
            Shape::Hexagon => write!(f, "Hexagon"),
            Shape::Heptagon => write!(f, "Heptagon"),
            Shape::Octagon => write!(f, "Octagon"),
        }
    }
}

impl Shape {
    pub fn edges(&self) -> u8 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Hexagon => 6,
            Shape::Heptagon => 7,
            Shape::Octagon => 8,
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "triangle" => Ok(Shape::Triangle),
            "square" => Ok(Shape::Square),
            "pentagon" => Ok(Shape::Pentagon),
            "hexagon" => Ok(Shape::Hexagon),
            "heptagon" => Ok(Shape::Heptagon),
            "octagon" => Ok(Shape::Octagon),
            _ => Err(format!("Unknown shape: {}", s)),
        }
    }
}

impl FromStr for Shape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Shape::parse(s)
    }
}
