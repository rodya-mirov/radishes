use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Deserializer};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TdTileSelect {
    None,
    Selected { x: i32, y: i32 },
}

impl Default for TdTileSelect {
    fn default() -> Self {
        TdTileSelect::None
    }
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
pub enum OwnedResource {
    Wood,
    Metal,
    Money,
}

impl std::fmt::Display for OwnedResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnedResource::Metal => write!(f, "Metal"),
            OwnedResource::Wood => write!(f, "Wood"),
            OwnedResource::Money => write!(f, "Money"),
        }
    }
}

pub const ALL_RESOURCES: &[OwnedResource] = &[
    OwnedResource::Wood,
    OwnedResource::Metal,
    OwnedResource::Money,
];

#[derive(Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct OwnedResources(pub BTreeMap<OwnedResource, i64>);

impl OwnedResources {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with(mut self, o: OwnedResource, amt: i64) -> Self {
        self.0.insert(o, amt);
        self
    }

    pub fn can_pay(&self, other: &OwnedResources) -> bool {
        for o in ALL_RESOURCES {
            let other_amt = other.0.get(o).copied().unwrap_or(0);
            if other_amt <= 0 {
                continue;
            }

            let my_amt = self.0.get(o).copied().unwrap_or(0);
            if my_amt < other_amt {
                return false;
            }
        }

        true
    }

    pub fn pay(&mut self, other: &OwnedResources) {
        for o in ALL_RESOURCES {
            let other_amt = other.0.get(o).copied().unwrap_or(0);

            *self.0.entry(*o).or_insert(0) -= other_amt;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Tile {
    Open,
    Wall,
    Spawn,
    Core,
}

mod deser {
    use serde::{
        de::{self, Visitor},
        Deserialize,
    };

    use super::*;

    impl<'de> Deserialize<'de> for Tile {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(TileDeserializer)
        }
    }

    struct TileDeserializer;

    impl<'de> Visitor<'de> for TileDeserializer {
        type Value = Tile;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing a tile")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                "Wall" => Ok(Tile::Wall),
                "Open" => Ok(Tile::Open),
                "Spawn" => Ok(Tile::Spawn),
                "Core" => Ok(Tile::Core),
                v => Err(E::custom(format!("Unrecognized value {} for Tile", v))),
            }
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v.as_str() {
                "Wall" => Ok(Tile::Wall),
                "Open" => Ok(Tile::Open),
                "Spawn" => Ok(Tile::Spawn),
                "Core" => Ok(Tile::Core),
                v => Err(E::custom(format!("Unrecognized value {} for Tile", v))),
            }
        }
    }
}

const DEFAULT_TILE: Tile = Tile::Wall;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Map {
    map: HashMap<(i32, i32), Tile>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: HashMap::new(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.map.get(&(x, y)).copied().unwrap_or(DEFAULT_TILE)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.map.insert((x, y), tile);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct TdCamera {
    /// Top pixel on camera
    pub top: i32,
    /// Leftmost pixel on camera
    pub left: i32,
}

#[derive(Deserialize, Clone, Eq, PartialEq)]
pub struct TileTransformDesc {
    pub source: Tile,
    pub target: Tile,
    pub cost: OwnedResources,
}

#[derive(Clone, Eq, PartialEq, Default)]
pub struct TileTransforms {
    map: HashMap<Tile, HashMap<Tile, OwnedResources>>,
}

impl TileTransforms {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, transform: TileTransformDesc) {
        self.map
            .entry(transform.source)
            .or_default()
            .insert(transform.target, transform.cost);
    }

    pub fn list_all_for(&self, source: Tile) -> HashMap<Tile, OwnedResources> {
        self.map
            .get(&source)
            .cloned()
            .unwrap_or_else(|| Default::default())
    }
}
