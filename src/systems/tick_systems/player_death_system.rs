//! System to detect and enforce player death

use legion::*;

use crate::resources::*;

#[system]
pub(super) fn player_death(#[resource] player_health: &mut PlayerHealth, #[resource] game_state: &mut GameState) {
    if player_health.health <= 0 {
        *game_state = GameState::Died;
    }
}
