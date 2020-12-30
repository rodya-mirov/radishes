use std::sync::Arc;

use yew::prelude::*;

use legion::*;

use crate::{assets::Assets, resources::*, ECS};

mod detail_view;
mod health_view;
mod launch_wave_view;
mod resource_view;
mod td_view;

pub struct GameView {
    _link: ComponentLink<Self>,
    ecs: ECS,
    assets: Arc<Assets>,
}

pub enum ModelMsg {}

// TODO this probably shouldn't live here
pub fn init_ecs(ecs: &ECS) {
    ecs.with(|world, r| {
        *r = Resources::default();
        world.clear();

        r.insert(NextWaveState::default());
        r.insert(PlayerHealth::default());

        r.insert(OwnedResources::new().with(OwnedResource::Money, 50).with(OwnedResource::Wood, 20));

        let mut map = Map::new();

        map.set_tile(8, 0, Tile::Spawn);
        map.set_tile(8, 1, Tile::Open);
        map.set_tile(8, 2, Tile::Open);
        map.set_tile(8, 3, Tile::Open);
        map.set_tile(7, 3, Tile::Open);
        map.set_tile(6, 3, Tile::Open);
        map.set_tile(5, 3, Tile::Open);
        map.set_tile(4, 3, Tile::Open);
        map.set_tile(0, 0, Tile::Spawn);
        map.set_tile(0, 1, Tile::Open);
        map.set_tile(0, 2, Tile::Open);
        map.set_tile(1, 2, Tile::Open);
        map.set_tile(2, 2, Tile::Open);
        map.set_tile(3, 2, Tile::Open);
        map.set_tile(4, 2, Tile::Open);
        map.set_tile(4, 1, Tile::Open);
        map.set_tile(4, 0, Tile::Open);
        map.set_tile(4, -1, Tile::Open);
        map.set_tile(4, -2, Tile::Core);

        r.insert(map);

        let mut camera = TdCamera::default();
        camera.top = -100;
        camera.left = -100;
        r.insert(camera);

        // TODO: probably put these in "raws" somewhere
        let mut transforms = TileTransforms::new();
        transforms.add(TileTransformDesc {
            source: Tile::Open,
            target: Tile::Wall,
            cost: OwnedResources::new().with(OwnedResource::Money, 5).with(OwnedResource::Wood, 5),
        });
        transforms.add(TileTransformDesc {
            source: Tile::Wall,
            target: Tile::Open,
            cost: OwnedResources::new().with(OwnedResource::Money, 3),
        });
        transforms.add(TileTransformDesc {
            source: Tile::Open,
            target: Tile::Spawn,
            cost: OwnedResources::new().with(OwnedResource::Metal, 15).with(OwnedResource::Wood, 25),
        });
        transforms.add(TileTransformDesc {
            source: Tile::Open,
            target: Tile::Core,
            cost: OwnedResources::new().with(OwnedResource::Metal, 15).with(OwnedResource::Wood, 25),
        });

        r.insert(transforms);
    })
}

#[derive(Properties, Clone)]
pub struct GameProps {
    pub ecs: ECS,
    pub assets: Arc<Assets>,
}

impl Component for GameView {
    type Message = ModelMsg;
    type Properties = GameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ecs: props.ecs,
            assets: props.assets,
            _link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.ecs = props.ecs;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="game-div">
                <div id="tower-defense-div">
                    <td_view::TowerDefenseComponent assets={self.assets.clone()} ecs={self.ecs.clone()} />
                </div>
                <div class="info-pane-main-div">
                    <health_view::HealthView ecs={self.ecs.clone()} />
                    <launch_wave_view::LaunchWaveView ecs={self.ecs.clone()} />
                    <resource_view::ResourceView ecs={self.ecs.clone()} />
                    <detail_view::DetailView ecs={self.ecs.clone()} />
                </div>
            </div>
        }
    }
}
