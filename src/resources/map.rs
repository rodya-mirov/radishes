use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Tile {
    Open,
    Wall,
    Spawn,
    Core,
}

const DEFAULT_TILE: Tile = Tile::Wall;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Map {
    map: HashMap<(i32, i32), Tile>,

    dijkstra_maps_dirty: bool,
    core_paths: DijkstraMap,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct DijkstraMap {
    // (x, y) -> cost, if set; None (no key) means inaccessible / no path
    costs: HashMap<(i32, i32), i32>,
}

impl DijkstraMap {
    fn new() -> Self {
        DijkstraMap { costs: HashMap::new() }
    }

    /// Recompute the path costs. Assumptions (which can be refactored out as lambdas as needed):
    /// - The objective tile is just "core tiles"
    /// - All travel is equally expensive (cost of 1)
    /// - Neighbors are just those that are directly adjacent (4 way)
    /// - Tiles are valid if and only if they are passable
    /// - "Default" tiles will never be given a weight
    fn recompute(&mut self, map: &HashMap<(i32, i32), Tile>) {
        #[derive(Eq, PartialEq, Debug, Copy, Clone)]
        struct NodeWeight {
            cost: i32,
            pos: (i32, i32),
        };

        // Note cost cmp reversed, so the heap will be a min heap
        impl Ord for NodeWeight {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost).then_with(|| self.pos.cmp(&other.pos))
            }
        }

        impl PartialOrd for NodeWeight {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut to_process: BinaryHeap<NodeWeight> = BinaryHeap::new();
        self.costs.clear();

        // first, go through and find every "zero;" for now this means cores
        for (pos, _) in map.iter().filter(|(_pos, tile)| **tile == Tile::Core) {
            let weight = NodeWeight { cost: 0, pos: *pos };
            to_process.push(weight);
        }

        while let Some(NodeWeight { cost, pos }) = to_process.pop() {
            if let Some(old_cost) = self.costs.get(&pos) {
                // Then we've already seen it and this was a redundant add; skip it and move on
                if *old_cost <= cost {
                    continue;
                }
            }

            self.costs.insert(pos, cost);

            if let Some(tile) = map.get(&pos) {
                if tile.is_passable() {
                    for neighbor_pos in neighbors(pos.0, pos.1).iter().copied() {
                        let nw = NodeWeight {
                            cost: cost + 1,
                            pos: neighbor_pos,
                        };
                        to_process.push(nw);
                    }
                }
            }
        }
    }
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: HashMap::new(),
            dijkstra_maps_dirty: false,
            core_paths: DijkstraMap::new(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.map.get(&(x, y)).copied().unwrap_or(DEFAULT_TILE)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.dijkstra_maps_dirty = true;
        self.map.insert((x, y), tile);
    }

    pub fn can_set_tile(&self, x: i32, y: i32, tile: Tile) -> bool {
        let mut test = self.clone();
        test.set_tile(x, y, tile);
        test.recompute_dijkstra_maps();

        for (pos, _) in test.map.iter().filter(|(_, tile)| **tile == Tile::Spawn) {
            if test.core_paths.costs.get(pos).is_none() {
                return false;
            }
        }

        true
    }

    fn recompute_dijkstra_maps(&mut self) {
        if !self.dijkstra_maps_dirty {
            return;
        }

        self.core_paths.recompute(&self.map);

        self.dijkstra_maps_dirty = false;
    }

    /// Get the tile coordinates of the best tile to move to, from here.
    /// If there is no improvement possible (either because you're "there" or because there's no
    /// path) just return the input.
    pub fn move_toward_spawn(&mut self, start_x: i32, start_y: i32) -> (i32, i32) {
        self.recompute_dijkstra_maps();

        let mut least_cost = self.core_paths.costs.get(&(start_x, start_y)).copied().unwrap_or(i32::max_value());
        let mut winning_coords = (start_x, start_y);

        for (x, y) in neighbors(start_x, start_y)
            .iter()
            .copied()
            .filter(|(x, y)| self.map.get(&(*x, *y)).copied().unwrap_or(DEFAULT_TILE).is_passable())
        {
            let cost = self.core_paths.costs.get(&(x, y)).copied().unwrap_or(i32::max_value());

            if cost < least_cost {
                least_cost = cost;
                winning_coords = (x, y);
            }
        }

        winning_coords
    }

    pub fn all_spawns(&self) -> Vec<(i32, i32)> {
        self.map
            .iter()
            .filter(|(_pos, tile)| **tile == Tile::Spawn)
            .map(|(pos, _)| *pos)
            .collect()
    }
}

impl Tile {
    fn is_passable(self) -> bool {
        match self {
            Tile::Open => true,
            Tile::Wall => false,
            Tile::Spawn => true,
            Tile::Core => true,
        }
    }
}

fn neighbors(x: i32, y: i32) -> [(i32, i32); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}
