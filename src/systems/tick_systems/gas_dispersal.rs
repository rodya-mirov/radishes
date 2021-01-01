//! Gas should spread from more-populated squares to less-populated squares

use legion::*;

use crate::resources::*;

#[system]
pub(super) fn disperse_gas(#[resource] map: &mut Map) {
    map.tick_gas_map();
}
