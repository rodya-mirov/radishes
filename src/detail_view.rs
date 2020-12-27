use yew::prelude::*;

use web_sys::MouseEvent;

use legion::Resources;

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
    ChangeTileButtonClicked {
        x: i32,
        y: i32,
        desired: Tile,
        costs: OwnedResources,
    },
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

    fn make_change_button(&self, resources: &Resources, x: i32, y: i32, tile: Tile, costs: OwnedResources) -> Html {
        let cost_display = self.make_cost_display(&costs);

        let can_pay = resources.get::<OwnedResources>().unwrap().can_pay(&costs);
        let can_path = resources.get::<Map>().unwrap().can_set_tile(x, y, tile);

        let would_work = can_pay && can_path;

        let click_cb = self.link.callback(move |_: MouseEvent| DetailViewMsg::ChangeTileButtonClicked {
            x,
            y,
            desired: tile,
            costs: costs.clone(),
        });

        let button_text = format!("Change to {:?}", tile);

        let style_class = format!(
            "change-tile-button {}",
            if would_work {
                "change-tile-button-enabled"
            } else {
                "change-tile-button-disabled"
            }
        );

        let pay_err = if can_pay {
            html! { <></> }
        } else {
            html! { <p> { "You cannot afford this." } </p> }
        };

        let path_err: Html = if can_path {
            html! { <></> }
        } else {
            html! { <p> { "This would block the exit." } </p> }
        };

        html! {
            <div onclick=click_cb class=style_class>
                <p> { &button_text } </p>
                { cost_display }
                { pay_err }
                { path_err }
            </div>
        }
    }

    fn make_cost_display(&self, cost: &OwnedResources) -> Html {
        cost.0
            .iter()
            .filter(|(_, amt)| **amt != 0)
            .map(|(o, amt)| {
                html! { <p> { &format!("{}: {}", o, amt) } </p> }
            })
            .collect()
    }

    fn tile_details(&self, x: i32, y: i32, tile: Tile) -> Html {
        let tile_str = format!("Selected tile at ({}, {}): {:?}", x, y, tile);

        // TODO: add a button to turn the tile into a different kind of tile
        // this should be achieved by launching a message to the ECS and having a system take care of it

        let mut changes: Vec<Html> = Vec::new();
        self.ecs.with(|_, r| {
            for (target, cost) in r.get::<TileTransforms>().unwrap().list_all_for(tile).into_iter() {
                changes.push(self.make_change_button(r, x, y, target, cost));
            }
        });

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
            DetailViewMsg::ChangeTileButtonClicked { x, y, desired, costs } => {
                self.ecs.with(|world, _| {
                    world.push((TryChangeTileType { x, y, desired, costs },));
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
