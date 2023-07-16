use std::fmt::Debug;
use std::path::PathBuf;

use clap::Parser;
use crate::map::{map_from_file, map_to_string};
use crate::path::solve;
use crate::tiles::Tile;

mod tiles;
mod path;
mod map;

#[derive(Parser, Debug)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut map = map_from_file(args.path);
    println!("{}", map_to_string(&map));
    println!();

    let path = solve(&map);
    for point in &path {
        map[point.1 as usize][point.0 as usize] = Tile::Path;
    }
    println!("{}", map_to_string(&map));
    println!("{} moves", &path.len() - 1);
}
