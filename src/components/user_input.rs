use legion::Entity;

use crate::resources::*;

/// Message component; the user has attempted to initiate a tile change
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TryChangeTileType {
    pub x: i32,
    pub y: i32,
    pub desired: Tile,
    pub costs: OwnedResources,
}

/// Message component; the user has attempted to initiate a structure build
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TryBuildStructure {
    pub x: i32,
    pub y: i32,
    pub desired: StructureKind,
    pub costs: OwnedResources,
}

/// Message component; the user has attempted to sell an existing structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrySellStructure {
    pub to_sell: Entity,
}

/// Message component; the user has attempted to initiate a new wave
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TryLaunchWave;

/// Message component; the user has attempted to initiate a new wave
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ToggleAutoLaunchWave;

/// Message component; the user has done a key thing regarding the canvas
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserKeyEvent {
    KeyDown(UserKey),
    KeyUp(UserKey),
    AllKeysUp,
}

/// What the canvas has determined the JS key event meant (e.g. w and arrow-up both become "up")
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserKey {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UserClickTile {
    pub tile_x: i32,
    pub tile_y: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UserUnselectTile;
