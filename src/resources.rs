use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TdMouseOver {
    None,
    MousedOver { x: i32, y: i32 },
}

impl Default for TdMouseOver {
    fn default() -> Self {
        TdMouseOver::None
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct OwnedResources {
    pub wood: i64,
    pub metal: i64,
    pub money: i64,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Open,
    Wall,
    Spawn,
    Core,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Map {
    map: HashMap<(i32, i32), Tile>,
    default_tile: Tile,
}

impl Map {
    pub fn new(default_tile: Tile) -> Self {
        Map {
            map: HashMap::new(),
            default_tile,
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.map.get(&(x, y)).copied().unwrap_or(self.default_tile)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.map.insert((x, y), tile);
    }
}
