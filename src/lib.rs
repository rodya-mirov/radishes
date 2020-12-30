use std::sync::Arc;

use wasm_bindgen::prelude::*;
use yew::{
    prelude::*,
    services::{IntervalService, Task},
};

use legion::Schedule;

mod assets;

pub use ecs_wrapper::ECS;

mod tile_helpers;

mod components;
mod resources;
mod systems;

mod canvas_util;

mod game_view;
mod new_game_view;

mod ecs_wrapper {
    use std::sync::{Arc, Mutex};

    use legion::{Resources, World};

    #[derive(Clone)]
    pub struct ECS(Arc<Mutex<(World, Resources)>>);

    impl ECS {
        pub fn new() -> Self {
            ECS(Arc::new(Mutex::new((World::default(), Resources::default()))))
        }

        pub fn with<A, F: FnOnce(&mut World, &mut Resources) -> A>(&self, f: F) -> A {
            let mut guard = self.0.lock().expect("ECS lock should be accessible");

            let (ref mut w, ref mut r) = &mut *guard;

            f(w, r)
        }
    }
}

struct View {
    ecs: ECS,
    schedule: Schedule,
    assets: Arc<assets::Assets>,

    // We have to keep a reference to this; it keeps triggering until it's dropped
    _tick_handle: Box<dyn Task>,
}

#[derive(Properties, Clone)]
struct ViewProps {
    assets: Arc<assets::Assets>,
}

enum ViewMsg {
    Tick,
}

impl Component for View {
    type Message = ViewMsg;
    type Properties = ViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ecs = ECS::new();

        game_view::init_ecs(&ecs);

        ecs.with(|_, r| {
            r.insert(resources::GameState::Opening);
        });

        let tick_cb = link.callback(|()| ViewMsg::Tick);

        let tick_handle = IntervalService::spawn(std::time::Duration::from_millis(50), tick_cb);

        let schedule = crate::systems::make_tick_schedule();

        Self {
            ecs,
            assets: props.assets,
            _tick_handle: Box::new(tick_handle),
            schedule,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        use resources::GameState;

        match msg {
            ViewMsg::Tick => {
                let schedule = &mut self.schedule;
                self.ecs.with(|world, resources| {
                    let should_run_tick = match *resources.get::<GameState>().unwrap() {
                        GameState::MainGame => true,
                        GameState::Opening | GameState::Died => false,
                    };

                    if should_run_tick {
                        schedule.execute(world, resources);
                    }
                });

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        use resources::GameState;

        let game_state = self.ecs.with(|_, r| *r.get::<GameState>().unwrap());

        match game_state {
            GameState::Opening => self.render_opening(),
            GameState::Died => self.render_died(),
            GameState::MainGame => self.render_main_game(),
        }
    }
}

impl View {
    fn render_opening(&self) -> Html {
        html! {
            <new_game_view::NewGameView ecs=self.ecs.clone() />
        }
    }

    fn render_died(&self) -> Html {
        html! {
            <new_game_view::DiedView ecs=self.ecs.clone() />
        }
    }

    fn render_main_game(&self) -> Html {
        html! {
            <game_view::GameView assets=self.assets.clone() ecs=self.ecs.clone() />
        }
    }
}

#[wasm_bindgen(start)]
pub async fn run_app() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let assets = assets::load_assets().await?;

    App::<View>::new().mount_to_body_with_props(ViewProps { assets: Arc::new(assets) });

    Ok(())
}
