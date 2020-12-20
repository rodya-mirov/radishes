use std::sync::{Arc, Mutex};

use legion::{Resources, World};

use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

mod components;
mod resources;
mod systems;

mod resource_view;
mod tower_defense;

type ECS = Arc<Mutex<(World, Resources)>>;

struct Model {
    _link: ComponentLink<Self>,
    _tick_handle: Box<dyn Task>,
    ecs: ECS,
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

        Self {
            ecs,
            _link: link,
            _tick_handle: Box::new(tick_handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModelMsg::Tick => {
                update_ecs(&self.ecs);
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
                <resource_view::ResourceView ecs={self.ecs.clone()} />
            </div>
        }
    }
}

fn make_ecs() -> ECS {
    let world = World::default();
    let mut r = Resources::default();

    r.insert(resources::Resources {
        money: 10,
        wood: 0,
        metal: 0,
    });

    Arc::new(Mutex::new((world, r)))
}

fn update_ecs(ecs: &ECS) {
    let (_, r) = &*ecs.lock().unwrap();
    r.get_mut::<crate::resources::Resources>().unwrap().money += 1;
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
