use crate::resources::*;

/// Message component; the user has attempted to initiate a tile change
/// TODO: should this be a resource instead? Maybe it makes more sense to just have a Queue of these things
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TryChangeTileType {
    pub x: i32,
    pub y: i32,
    pub desired: Tile,
    pub costs: OwnedResources,
}

/// Component indicating the entity has a world position in pixels
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    // Note -- these are "world pixel" coordinates, not tile coordinates or etc.
    pub x: i32,
    pub y: i32,
}

/// Tag component, indicating a component is a tower defense mob
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TdMob;

/// Indication of how a positional object should be rendered
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Renderable {
    Circle { radius: i32 },
}
