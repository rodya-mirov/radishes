use crate::resources::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TryChangeTileType {
    pub x: i32,
    pub y: i32,
    pub desired: Tile,
    pub costs: OwnedResources,
}
