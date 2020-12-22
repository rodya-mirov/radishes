use yew::prelude::*;

use crate::{resources::*, ECS};

pub(crate) struct DetailView {
    detail_state: DetailState,
}

#[derive(Clone, Properties)]
pub(crate) struct DetailViewProps {
    pub(crate) ecs: ECS,
}

#[derive(Clone)]
pub(crate) enum DetailViewMsg {}

enum DetailState {
    Nothing,
    Tile { x: i32, y: i32, tile: Tile },
}

fn from_ecs(ecs: &ECS) -> DetailState {
    let mut ecs = ecs.lock().unwrap();
    let (_, r) = &mut *ecs;

    let mouseover = *(r.get_or_default::<TdMouseOver>());

    let out = match mouseover {
        TdMouseOver::None => DetailState::Nothing,
        TdMouseOver::MousedOver { x, y } => {
            let tile = r.get::<Map>().unwrap().get_tile(x, y);
            DetailState::Tile { x, y, tile }
        }
    };

    out
}

impl DetailView {
    fn update_state(&mut self, ecs: &ECS) -> bool {
        self.detail_state = from_ecs(ecs);
        true
    }
}

impl Component for DetailView {
    type Message = DetailViewMsg;
    type Properties = DetailViewProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // link is not used; no callbacks are needed
        DetailView {
            detail_state: from_ecs(&props.ecs),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {}
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.update_state(&props.ecs)
    }

    fn view(&self) -> Html {
        match &self.detail_state {
            DetailState::Nothing => empty_pane(),
            DetailState::Tile { x, y, tile } => tile_details(*x, *y, *tile),
        }
    }
}

fn empty_pane() -> Html {
    html! {
        <></>
    }
}

fn tile_details(x: i32, y: i32, tile: Tile) -> Html {
    let tile_str = format!("Selected tile at ({}, {}): {:?}", x, y, tile);
    html! {
        <div class="info-pane">
            <p>{ tile_str }</p>
        </div>
    }
}
