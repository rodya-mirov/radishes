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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct GasFlow;

trait PassableChecker {
    fn is_passable(&self, tile: Tile) -> bool;
}

impl PassableChecker for GasFlow {
    fn is_passable(&self, tile: Tile) -> bool {
        match tile {
            Tile::Open => true,
            Tile::Wall => false,
            Tile::Spawn => false,
            Tile::Core => false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Map {
    map: HashMap<(i32, i32), Tile>,

    dijkstra_maps_dirty: bool,
    core_paths: DijkstraMap,

    poison_gas_map: FlowMap<GasFlow>,
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
            poison_gas_map: FlowMap::new(GasFlow, 6, 1),
        }
    }

    pub fn tick_gas_map(&mut self) {
        self.poison_gas_map.tick(&self.map, DEFAULT_TILE);
    }

    pub fn add_gas_to_tile(&mut self, tile_x: i32, tile_y: i32, amount: i32) {
        self.poison_gas_map.add_amount(tile_x, tile_y, amount)
    }

    pub fn get_gas_amount(&self, tile_x: i32, tile_y: i32) -> i32 {
        self.poison_gas_map.amounts.get(&(tile_x, tile_y)).copied().unwrap_or(0)
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct FlowMap<P: PassableChecker + Clone + 'static> {
    amounts: HashMap<(i32, i32), i32>,
    tile_checker: P,
    // i32 for simplicity but should be nonnegative
    // this is the amount of fluid that can flow out of any particular square per tick
    fluidity: i32,
    // i32 for simplicity but should be nonnegative
    // this is the amount that is deleted from each square, each tick (before flow)
    // note that if this is zero, but there is a source somewhere, the map will eventually flood
    dispersal: i32,
}

impl<P: PassableChecker + Clone + 'static> FlowMap<P> {
    pub fn new(tile_checker: P, fluidity: i32, dispersal: i32) -> FlowMap<P> {
        Self {
            amounts: HashMap::new(),
            tile_checker,
            fluidity,
            dispersal,
        }
    }

    pub fn tick(&mut self, tiles: &HashMap<(i32, i32), Tile>, default_tile: Tile) {
        self.disperse();
        self.flow(tiles, default_tile);
        self.cleanup();
    }

    fn disperse(&mut self) {
        let dispersal = self.dispersal;

        self.amounts.values_mut().for_each(|amt| {
            *amt = (*amt - dispersal).max(0);
        });

        self.cleanup();
    }

    /// Most complex part; each tile has an amount on it, and a fluidity
    /// Basically any tile that has any remaining fluidity can shift a gas unit to a
    /// tile which has fewer gas units
    fn flow(&mut self, tiles: &HashMap<(i32, i32), Tile>, default_tile: Tile) {
        let mut flow_from: Vec<(i32, i32)> = tiles.keys().copied().collect();
        // We have to sort because this sloppy algorithm is not commutative (because we don't iterate)
        // and we don't want to like, refresh the page and now the player's gas traps have different coverage
        flow_from.sort();

        // In whatever order we have these tiles in, just
        for (tile_x, tile_y) in flow_from {
            let mut neighbors: Vec<(i32, i32)> = neighbors(tile_x, tile_y)
                .iter()
                .copied()
                .filter(|(x, y)| {
                    let this_tile = tiles.get(&(*x, *y)).copied().unwrap_or(default_tile);
                    self.tile_checker.is_passable(this_tile)
                })
                .collect();

            let mut remaining_fluidity = self.fluidity;
            let mut neighbor_index = 0;
            let mut self_amount = self.amounts.get(&(tile_x, tile_y)).copied().unwrap_or(0);

            while remaining_fluidity > 0 && !neighbors.is_empty() && self_amount > 1 {
                neighbor_index = neighbor_index % neighbors.len();

                let (nx, ny) = neighbors[neighbor_index];

                let neighbor = self.amounts.entry((nx, ny)).or_insert(0);

                if *neighbor + 1 < self_amount {
                    self_amount -= 1;
                    remaining_fluidity -= 1;
                    *neighbor += 1;
                    neighbor_index += 1;
                } else {
                    neighbors.remove(neighbor_index);
                }
            }

            self.amounts.insert((tile_x, tile_y), self_amount);
        }
    }

    /// Delete unused squares to save space
    fn cleanup(&mut self) {
        self.amounts.retain(|_, amt| *amt != 0)
    }

    pub fn add_amount(&mut self, tile_x: i32, tile_y: i32, to_add: i32) {
        let amount = self.amounts.entry((tile_x, tile_y)).or_insert(0);
        *amount = (*amount).saturating_add(to_add);
    }
}
