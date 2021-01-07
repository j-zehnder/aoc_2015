extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
extern crate scan_fmt;

pub mod d01; // elevator
pub mod d02; // wrapping presents
pub mod d03; // delivery directions
pub mod d04; // md5 hashes
pub mod d05; // nice strings
pub mod d06; // million light grid
pub mod d07; // bitwise logic gates
pub mod d08; // code vs string length
pub mod d09; // tsp
pub mod d13; // optimal seating arrangement
pub mod d14; // reindeer olympics

aoc_lib! { year = 2015 }
