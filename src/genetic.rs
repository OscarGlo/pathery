use rand::{RngCore, thread_rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::map::Map;
use crate::path::{Point, solve};
use crate::tiles::Tile;

pub type Solution = Vec<Point>;

pub fn valid_walls(map: &Map) -> Solution {
    return map.iter().enumerate().flat_map(|(y, r)| {
        r.iter().enumerate()
            .filter(|(_, t)| t == &&Tile::Empty)
            .map(move |(x, _)| (x as i8, y as i8))
    }).collect::<Solution>();
}

pub fn random_solution(valid_walls: &Solution, wall_count: usize) -> Solution {
    let mut walls = valid_walls.clone();
    walls.shuffle(&mut thread_rng());
    return walls[..wall_count].iter().map(|p| *p).collect::<Solution>();
}

pub type Population = Vec<Solution>;

pub fn init_population(map: &Map, size: usize, wall_count: usize) -> Population {
    let valid_walls = valid_walls(map);
    return (0..size).into_iter().map(|_| random_solution(&valid_walls, wall_count)).collect();
}

pub fn fitness(map: &Map, solution: &Solution) -> u16 {
    // Place walls
    let mut working_map = map.clone();
    for point in solution {
        working_map[point.1 as usize][point.0 as usize] = Tile::Wall;
    }

    // Get path length
    let result = solve(&working_map);
    if result.is_none() {
        return 0;
    }
    let path = result.unwrap();
    return path.len() as u16;
}

fn get_parent(population: &Population, rng: &mut ThreadRng) -> Solution {
    let mut parent = population.choose(rng).unwrap().clone();
    parent.shuffle(rng);
    return parent;
}

fn splice(parent1: &Solution, parent2: &Solution, mutation_treshold: u32, valid_walls: &Solution, rng: &mut ThreadRng) -> Solution {
    let len = parent1.len() / 2;
    let mut child = parent1[..len].to_vec();
    child.extend(parent2[len..].iter());
    if rng.next_u32() > mutation_treshold {
        child.shuffle(rng);
        while rng.next_u32() > mutation_treshold {
            child.pop();
            child.push(valid_walls.choose(rng).unwrap().clone())
        }
    }
    return child;
}

pub fn generation(map: &Map, mut population: Population, elite_count: usize, mutation_rate: f32) -> Population {
    let mut rng = thread_rng();
    let valid_walls = valid_walls(&map);
    let mutation_treshold = (mutation_rate * (u32::MAX as f32)) as u32;

    population.sort_by_cached_key(|s| fitness(map, s));
    population.reverse();

    // Keep best solutions
    let mut next = population[..elite_count].to_vec();

    // Fill rest of population with random splits
    let crossover_count = (population.len() - elite_count) / 2;
    for _ in 0..crossover_count {
        let parent1 = get_parent(&population, &mut rng);
        let parent2 = get_parent(&population, &mut rng);

        next.push(splice(&parent1, &parent2, mutation_treshold, &valid_walls, &mut rng));
        next.push(splice(&parent2, &parent1, mutation_treshold, &valid_walls, &mut rng));
    }

    return next;
}