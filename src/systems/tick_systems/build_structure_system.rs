use legion::{systems::CommandBuffer, world::SubWorld, *};

use crate::tile_helpers::{TILE_HEIGHT_PIXELS, TILE_WIDTH_PIXELS};
use crate::{components::*, resources::*};

#[system]
#[read_component(TryBuildStructure)]
pub(super) fn build_structures(#[resource] owned_resources: &mut OwnedResources, cmd: &mut CommandBuffer, world: &SubWorld) {
    let mut query = <(Entity, Read<TryBuildStructure>)>::query();

    for (entity, try_build) in query.iter(world) {
        cmd.remove(*entity);

        let &TryBuildStructure {
            x: tile_x,
            y: tile_y,
            desired,
            ref costs,
        } = try_build;

        if owned_resources.can_pay(costs) {
            owned_resources.pay(costs);

            match desired {
                StructureKind::GasTrap => build_gas_trap(cmd, tile_x, tile_y),
            }
        }
    }
}

fn build_gas_trap(cmd: &mut CommandBuffer, tile_x: i32, tile_y: i32) {
    cmd.push((
        Position::at_tile_center(tile_x, tile_y),
        Structure(StructureKind::GasTrap),
        PoisonGasTrap { amount: 10 },
        Renderable::Bitmap {
            dx: -TILE_WIDTH_PIXELS / 2,
            dy: -TILE_HEIGHT_PIXELS / 2,
            bitmap: RenderBitmap::GasTrap,
        },
        // TODO: sell value should be tracked in a resource or something somewhere
        SellValue(OwnedResources::new().with(OwnedResource::Money, 10).with(OwnedResource::Wood, 5)),
    ));
}
