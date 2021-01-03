use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::{components::*, resources::*};

#[system]
#[read_component(UserClickTile)]
#[read_component(UserUnselectTile)]
#[read_component(Structure)]
#[read_component(SellValue)]
#[read_component(Position)]
pub(super) fn process_tile_clicks(#[resource] selected_tile: &mut TdTileSelect, cmd: &mut CommandBuffer, world: &SubWorld) {
    let mut query = <(Entity, Read<UserClickTile>)>::query();

    for (entity, click_tile) in query.iter(world) {
        let &UserClickTile { tile_x, tile_y } = click_tile;

        // TODO: get all the structures here, too
        *selected_tile = TdTileSelect::Selected {
            x: tile_x,
            y: tile_y,
            structures: vec![],
        };

        cmd.remove(*entity);
    }

    let mut query = <(Entity, Read<UserUnselectTile>)>::query();
    for (entity, _) in query.iter(world) {
        *selected_tile = TdTileSelect::None;
        cmd.remove(*entity);
    }

    if let TdTileSelect::Selected {
        x: tile_x,
        y: tile_y,
        structures,
    } = selected_tile
    {
        structures.clear();

        for (entity, structure, pos, maybe_sell_value) in
            <(Entity, Read<Structure>, Read<Position>, TryRead<SellValue>)>::query().iter(world)
        {
            let (pos_tile_x, pos_tile_y) = pos.to_tile_coords();
            if (pos_tile_x, pos_tile_y) != (*tile_x, *tile_y) {
                continue;
            }

            structures.push(SelectedStructure {
                entity: *entity,
                kind: structure.0,
                sell_value: maybe_sell_value.map(|sv| sv.0.clone()),
            });
        }
    }
}
