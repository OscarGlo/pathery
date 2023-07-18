use crate::map::Map;
use crate::tiles::Tile;

pub type Point = (i8, i8);
pub type Path = Vec<Point>;

#[derive(Debug)]
struct Node {
    point: Point,
    cost: u8,
    heuristic: u8,
    path: Path,
}

fn find_tile(map: &Map, tile: &Tile) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for (y, r) in map.iter().enumerate() {
        for (x, t) in r.iter().enumerate() {
            if t == tile {
                points.push((x as i8, y as i8));
            }
        }
    }
    return points;
}

fn shortest_path(map: &Map, start: Option<Point>, target: Tile) -> Option<Path> {
    let end_pos = find_tile(map, &target);

    let heuristic = |p: &Point| {
        return end_pos.iter().map(|e| p.0.abs_diff(e.0) + p.1.abs_diff(e.1)).min().unwrap();
    };

    let step = |node: &Node, map: &Map, direction: Point| -> Option<Node> {
        let p = (node.point.0 + direction.0, node.point.1 + direction.1);

        // Check bounds
        if p.0 < 0 || p.0 >= map[0].len() as i8 || p.1 < 0 || p.1 >= map.len() as i8 {
            return None;
        }

        let mut path = node.path.clone();
        path.push(p.clone());

        return Some(Node {
            point: p,
            cost: node.cost + 1,
            heuristic: heuristic(&p),
            path,
        });
    };

    let mut to_search: Vec<Node>;
    if start.is_some() {
        // Init search at start posititon
        let p = start.unwrap();
        to_search = vec!(Node {
            point: p,
            cost: 0,
            heuristic: heuristic(&p),
            path: vec![p.clone()],
        })
    } else {
        // Init search at every start tile
        let start_pos = find_tile(map, &Tile::Start);
        to_search = start_pos.iter().map(|p| Node {
            point: p.clone(),
            cost: 0,
            heuristic: heuristic(p),
            path: vec![p.clone()],
        }).collect::<Vec<Node>>();
    }

    let mut searched = vec![];

    while !to_search.is_empty() {
        let current = to_search.pop().unwrap();
        let point = current.point;

        if map[point.1 as usize][point.0 as usize] == target {
            return Some(current.path);
        }

        searched.push(point);

        // Loop through all neighbors and add valid and not visited tiles
        'nei: for d in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let tmp = step(&current, map, d);
            if tmp.is_none() {
                continue
            }

            let mut node = tmp.unwrap();

            let mut tile = map[node.point.1 as usize][node.point.0 as usize];

            // TODO: Fix direction priorities
            if searched.contains(&node.point) ||
                tile == Tile::Wall ||
                to_search.iter().find(|m| m.point == node.point && m.cost <= node.cost).is_some()
            {
                continue;
            }

            // Slide on ice tiles
            while tile == Tile::Ice {
                let tmp = step(&node, map, d);
                if tmp.is_none() {
                    continue 'nei;
                }

                node = tmp.unwrap();
                tile = map[node.point.1 as usize][node.point.0 as usize];

                if tile == Tile::Wall {
                    continue 'nei;
                }
            }

            let value = node.cost + node.heuristic;
            for i in 0..to_search.len() {
                if to_search[i].cost + to_search[i].heuristic <= value {
                    to_search.insert(i, node);
                    continue 'nei;
                }
            }
            to_search.push(node);
        }
    }
    return None;
}

fn get_checkpoint_id(tile: &Tile) -> u8 {
    match tile {
        Tile::Checkpoint(id) => id.clone(),
        _ => panic!("Tile::Checkpoint expected")
    }
}

// TODO: Implement teleporters
pub fn solve(map: &Map) -> Option<Path> {
    let mut checkpoints = map.iter()
        .flat_map(|r| r.iter()
            .filter(|t| matches!(t, &Tile::Checkpoint(_)))
            .map(get_checkpoint_id)
        ).collect::<Vec<u8>>();
    checkpoints.sort();
    checkpoints.dedup();

    let mut path: Vec<Point> = vec![];
    for id in checkpoints {
        let p = path.pop();
        let part = shortest_path(map, p, Tile::Checkpoint(id));
        if part.is_none() {
            return None
        }
        path.append(&mut part.unwrap());
    }
    let p = path.pop();
    let part = shortest_path(map, p, Tile::Exit);
    if part.is_none() {
        return None
    }
    path.append(&mut part.unwrap());

    return Some(path);
}