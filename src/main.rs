use std::fmt::Debug;
use std::path::PathBuf;

use clap::Parser;

use crate::genetic::{fitness, generation, init_population};
use crate::map::{map_from_file, map_to_string};
use crate::path::solve;
use crate::tiles::Tile;

mod tiles;
mod path;
mod map;
mod genetic;

#[derive(Parser, Debug)]
struct Args {
    path: PathBuf,
    walls: usize,
}

fn main() {
    let args = Args::parse();
    let mut map = map_from_file(args.path);
    println!("{}", map_to_string(&map));
    println!();

    let mut population = init_population(&map, 128, args.walls);
    for epoch in 1..=25 {
        for _ in 1..=100 {
            population = generation(&map, population, 24, 0.5);
        }
        println!("Epoch {}: {}", epoch, fitness(&map, &population[0]));
    }

    let best = &population[0];
    for p in best {
        map[p.1 as usize][p.0 as usize] = Tile::Wall;
    }
    println!("{}", map_to_string(&map));
    println!();

    let path = solve(&map).expect("No valid solution found");
    for p in path {
        map[p.1 as usize][p.0 as usize] = Tile::Path;
    }
    println!("{}", map_to_string(&map));
}
