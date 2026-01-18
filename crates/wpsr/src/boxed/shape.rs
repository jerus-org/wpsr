use std::{fmt::Display, str::FromStr};

use clap::Parser;

use crate::Error;

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

    pub fn parse(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "triangle" => Ok(Shape::Triangle),
            "square" => Ok(Shape::Square),
            "pentagon" => Ok(Shape::Pentagon),
            "hexagon" => Ok(Shape::Hexagon),
            "heptagon" => Ok(Shape::Heptagon),
            "octagon" => Ok(Shape::Octagon),
            _ => Err(Error::UnknownShape(s.to_string())),
        }
    }

    pub fn from_edges(edges: u8) -> Result<Self, Error> {
        match edges {
            3 => Ok(Shape::Triangle),
            4 => Ok(Shape::Square),
            5 => Ok(Shape::Pentagon),
            6 => Ok(Shape::Hexagon),
            7 => Ok(Shape::Heptagon),
            8 => Ok(Shape::Octagon),
            _ => Err(Error::UnknownShapeForEdges(edges)),
        }
    }
}

impl FromStr for Shape {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Shape::parse(s)
    }
}
