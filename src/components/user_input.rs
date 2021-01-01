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

/// Message component; the user has attempted to initiate a new wave
/// TODO: should this be a resource instead? Maybe it makes more sense to just have a Queue of these things
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TryLaunchWave;

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
