use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Ice,
    Wall,
    Start,
    Exit,
    Checkpoint(u8),
    In(u8),
    Out(u8),
    Path,
}

pub fn char_tile_map() -> HashMap<char, Tile> {
    return HashMap::from([
        (' ', Tile::Empty),
        ('_', Tile::Ice),
        ('#', Tile::Wall),
        ('-', Tile::Start),
        ('+', Tile::Exit),
        ('A', Tile::Checkpoint(0)),
        ('B', Tile::Checkpoint(1)),
        ('C', Tile::Checkpoint(2)),
        ('D', Tile::Checkpoint(3)),
        ('E', Tile::Checkpoint(4)),
        ('(', Tile::In(0)),
        (')', Tile::Out(0)),
        ('{', Tile::In(1)),
        ('}', Tile::Out(1)),
        ('[', Tile::In(2)),
        (']', Tile::Out(2)),
        ('<', Tile::In(2)),
        ('>', Tile::Out(2)),
    ]);
}