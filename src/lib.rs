use legion::{Resources, Schedule, World};

use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::services::{IntervalService, Task};

mod tile_helpers;

mod components;
mod resources;
mod systems;

mod canvas_util;

mod detail_view;
mod resource_view;
mod tower_defense;

mod ecs_wrapper {

    use std::sync::{Arc, Mutex};

    use legion::{Resources, World};

    #[derive(Clone)]
    pub struct ECS(Arc<Mutex<(World, Resources)>>);

    impl ECS {
        pub fn new(world: World, resources: Resources) -> Self {
            ECS(Arc::new(Mutex::new((world, resources))))
        }

        pub fn with<A, F: FnOnce(&mut World, &mut Resources) -> A>(&self, f: F) -> A {
            let mut guard = self.0.lock().expect("ECS lock should be accessible");

            let (ref mut w, ref mut r) = &mut *guard;

            f(w, r)
        }
    }
}

pub use ecs_wrapper::ECS;

struct Model {
    _link: ComponentLink<Self>,
    _tick_handle: Box<dyn Task>,
    ecs: ECS,
    schedule: Schedule,
}

enum ModelMsg {
    Tick,
}

impl Component for Model {
    type Message = ModelMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ecs = make_ecs();

        let tick_cb = link.callback(|()| {
            // uncomment to verify that in fact the ticker IS working
            // ConsoleService::log("tick Tick TICK");
            ModelMsg::Tick
        });
        let tick_handle = IntervalService::spawn(std::time::Duration::from_millis(50), tick_cb.clone());

        let schedule = systems::make_tick_schedule();

        Self {
            ecs,
            schedule,
            _link: link,
            _tick_handle: Box::new(tick_handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModelMsg::Tick => {
                let schedule = &mut self.schedule;
                self.ecs.with(|world, resources| {
                    schedule.execute(world, resources);
                });

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="game-div">
                <div id="tower-defense-div">
                    <tower_defense::TowerDefenseComponent ecs={self.ecs.clone()} />
                </div>
                <div class="info-pane-main-div">
                    <resource_view::ResourceView ecs={self.ecs.clone()} />
                    <detail_view::DetailView ecs={self.ecs.clone()} />
                </div>
            </div>
        }
    }
}

fn make_ecs() -> ECS {
    use {components::*, resources::*};

    let mut world = World::default();

    for x in (-10)..15 {
        for y in -10..15 {
            world.push((
                Position {
                    x: x * 30 + 15,
                    y: y * 30 + 15,
                },
                TdMob,
                Renderable::Circle { radius: 10 },
            ));
        }
    }

    let mut r = Resources::default();

    r.insert(OwnedResources::new().with(OwnedResource::Money, 50).with(OwnedResource::Wood, 20));

    let mut map = Map::new();

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

    ECS::new(world, r)
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
