use yew::prelude::*;

use web_sys::MouseEvent;

use crate::{components::*, resources::*, ECS};

pub(crate) struct DetailView {
    link: ComponentLink<Self>,
    ecs: ECS,
    detail_state: DetailState,
}

#[derive(Clone, Properties)]
pub(crate) struct DetailViewProps {
    pub(crate) ecs: ECS,
}

#[derive(Clone)]
pub(crate) enum DetailViewMsg {
    ChangeTileButtonClicked { x: i32, y: i32, desired: Tile },
}

enum DetailState {
    Nothing,
    Tile { x: i32, y: i32, tile: Tile },
}

fn from_ecs(ecs: &ECS) -> DetailState {
    ecs.with(|_, r| {
        let mouseover = *(r.get_or_default::<TdTileSelect>());

        match mouseover {
            TdTileSelect::None => DetailState::Nothing,
            TdTileSelect::Selected { x, y } => {
                let tile = r.get::<Map>().unwrap().get_tile(x, y);
                DetailState::Tile { x, y, tile }
            }
        }
    })
}

impl DetailView {
    fn update_state(&mut self, ecs: &ECS) -> bool {
        self.detail_state = from_ecs(ecs);
        true
    }

    fn make_change_button(&self, x: i32, y: i32, tile: Tile) -> Html {
        let click_cb =
            self.link.callback(
                move |_: MouseEvent| DetailViewMsg::ChangeTileButtonClicked {
                    x,
                    y,
                    desired: tile,
                },
            );

        let button_text = format!("Change to {:?}", tile);

        html! {
            <div onclick=click_cb class="change-tile-button">
                <p> { &button_text } </p>
            </div>
        }
    }

    fn tile_details(&self, x: i32, y: i32, tile: Tile) -> Html {
        let tile_str = format!("Selected tile at ({}, {}): {:?}", x, y, tile);

        // TODO: add a button to turn the tile into a different kind of tile
        // this should be achieved by launching a message to the ECS and having a system take care of it

        let mut changes: Vec<Html> = Vec::new();
        match tile {
            Tile::Wall => {
                changes.push(self.make_change_button(x, y, Tile::Open));
            }
            Tile::Open => {
                changes.push(self.make_change_button(x, y, Tile::Wall));
            }
            // no mods available for special tiles
            Tile::Core | Tile::Spawn => {}
        }

        html! {
            <div class="info-pane">
                <p>{ tile_str }</p>
                { changes }
            </div>
        }
    }
}

impl Component for DetailView {
    type Message = DetailViewMsg;
    type Properties = DetailViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DetailView {
            link,
            detail_state: from_ecs(&props.ecs),
            ecs: props.ecs,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            DetailViewMsg::ChangeTileButtonClicked { x, y, desired } => {
                self.ecs.with(|world, _| {
                    world.push((TryChangeTileType { x, y, desired },));
                });
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.update_state(&props.ecs)
    }

    fn view(&self) -> Html {
        match &self.detail_state {
            DetailState::Nothing => empty_pane(),
            DetailState::Tile { x, y, tile } => self.tile_details(*x, *y, *tile),
        }
    }
}

fn empty_pane() -> Html {
    html! {
        <></>
    }
}
