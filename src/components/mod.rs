use legion::*;

use crate::resources::*;

mod user_input;

pub use user_input::*;

/// Indicates the entity has touched the core.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TouchedCore;

/// Component indicating the entity has a world position in pixels
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    // Note -- these are "world pixel" coordinates, not tile coordinates or etc.
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn at_tile_center(tile_x: i32, tile_y: i32) -> Position {
        let (x, y) = crate::tile_helpers::tile_to_pixel_coords(tile_x, tile_y);
        Position { x, y }
    }

    pub fn to_tile_coords(&self) -> (i32, i32) {
        crate::tile_helpers::coords_to_tile(self.x, self.y)
    }
}

/// Tag component, indicating a component is a tower defense mob
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TdMob;

/// Component indicating the entity has health. Probably they can take damage and if the health
/// goes to zero, they'll die.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MobHealth {
    pub current_health: i32,
    pub max_health: i32,
}

/// Tag component indicating the entity has died and should be deleted and their death events handled properly
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Died;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnDeath {
    // TODO: smallvec almost everywhere
    pub events: Vec<DeathEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeathEvent {
    GetResources(OwnedResource, i64),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Renderable {
    #[allow(unused)] // we are going to use this at some point, the code is tested
    Bitmap {
        dx: i32,
        dy: i32,
        bitmap: RenderBitmap,
    },
    Geometry(RenderGeometry),
}

/// Options for rendering an object using a bitmap in the Assets folder
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderBitmap {
    GasTrap,
}

/// Options for rendering an object using geometry
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderGeometry {
    Circle { radius: i32 },
}

/// Indication of the state of a wave associated to the given entity.
/// e.g. wave 3, active; or wave 2, spawn timer remaining 15 ticks
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WaveState {
    pub wave_num: usize,
    pub wait_state: WaitState,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WaitState {
    Active,
    Waiting { ticks_remaining: usize },
}

/// Indication that an otherwise renderable entity should not be rendered
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hidden;

/// Indicates the entity needs air; this has a variety of implications
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Breathes;

/// Tag component indicating the entity is a structure
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Structure(pub StructureKind);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SellValue(pub OwnedResources);

/// Indicates this is a Gas Trap, and therefore emanates poison gas every tick
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PoisonGasTrap {
    /// How much gas is produced each tick
    pub amount: i32,
}

/// Indicates the target should take a certain amount of damage. Can be expanded for damage type,
/// source, etc. so we can do all resistances, callbacks, particles, and so on in one place.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TakeDamage {
    pub target: Entity,
    pub amount: i32,
}
