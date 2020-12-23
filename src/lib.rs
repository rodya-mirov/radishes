use legion::{Resources, Schedule, World};

use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::services::{IntervalService, Task};

mod components;
mod resources;
mod systems;

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
            let mut guard = self.0.lock().unwrap();

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
        let tick_handle =
            IntervalService::spawn(std::time::Duration::from_millis(50), tick_cb.clone());

        let schedule = systems::make_schedule();

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
    use resources::*;

    let world = World::default();
    let mut r = Resources::default();

    r.insert(OwnedResources {
        money: 10,
        wood: 0,
        metal: 0,
    });

    let mut map = Map::new(Tile::Wall);

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

    r.insert(TdCamera::default());

    ECS::new(world, r)
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
