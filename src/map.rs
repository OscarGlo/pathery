use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::tiles::*;

pub type Map = Vec<Vec<Tile>>;

pub fn map_from_file(path: PathBuf) -> Map {
    let char_tiles = char_tile_map();

    let content = fs::read_to_string(path).expect("Error reading file");

    let width = content.lines().map(|l| l.len()).max().unwrap_or(0);

    return content.lines().map(|l| {
        let padded = format!("{:width$}", l, width = width);
        return padded.chars().map(|c| char_tiles[&c]).collect();
    }).collect();
}

pub fn map_to_string(map: &Map) -> String {
    let char_tiles = char_tile_map();
    let mut tile_chars: HashMap<&Tile, &char> = char_tiles.iter()
        .map(|(k, v)| (v, k)).collect();
    tile_chars.insert(&Tile::Path, &'*');

    return map.iter().map(|r|
        r.iter().map(|t| tile_chars[t].clone()).collect::<String>()
    ).collect::<Vec<String>>().join("\n");
}