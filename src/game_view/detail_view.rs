use yew::prelude::*;

use web_sys::MouseEvent;

use legion::*;

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
    // TODO: I don't like tracking the costs in the view layer
    ChangeTileButtonClicked {
        x: i32,
        y: i32,
        desired: Tile,
        costs: OwnedResources,
    },
    BuildStructureButtonClicked {
        x: i32,
        y: i32,
        desired: StructureKind,
        costs: OwnedResources,
    },
    SellExistingStructureButtonClicked {
        to_sell: Entity,
    },
    Nothing,
}

enum DetailState {
    Nothing,
    Tile {
        x: i32,
        y: i32,
        tile: Tile,
        structures: Vec<StructureState>,
    },
}

struct StructureState {
    entity: Entity,
    kind: StructureKind,
    sell_value: Option<OwnedResources>,
}

fn from_ecs(ecs: &ECS) -> DetailState {
    ecs.with(|_, r| {
        let mouseover = r.get_or_default::<TdTileSelect>().clone();

        match mouseover {
            TdTileSelect::None => DetailState::Nothing,
            TdTileSelect::Selected {
                x,
                y,
                structures: selected,
            } => {
                let tile = r.get::<Map>().unwrap().get_tile(x, y);
                let structures: Vec<StructureState> = selected
                    .iter()
                    .map(|s| StructureState {
                        entity: s.entity,
                        kind: s.kind,
                        sell_value: s.sell_value.clone(),
                    })
                    .collect();
                DetailState::Tile { x, y, tile, structures }
            }
        }
    })
}

impl DetailView {
    fn update_state(&mut self, ecs: &ECS) -> bool {
        self.detail_state = from_ecs(ecs);
        true
    }

    fn make_structure_view(&self, structure: &StructureState) -> Html {
        let cost_display = if let Some(sv) = structure.sell_value.as_ref() {
            self.make_cost_display(sv)
        } else {
            html! {}
        };

        let can_sell = structure.sell_value.is_some();

        let would_work = can_sell;

        let click_cb = if would_work {
            let to_sell = structure.entity;
            self.link
                .callback(move |_: MouseEvent| DetailViewMsg::SellExistingStructureButtonClicked { to_sell })
        } else {
            self.link.callback(|_: MouseEvent| DetailViewMsg::Nothing)
        };

        let trap_name = match structure.kind {
            StructureKind::GasTrap => "Gas Trap",
        };

        let button_text = format!("Sell existing {}", trap_name);

        let style_class = format!(
            "build-button {}",
            if would_work {
                "build-button-enabled"
            } else {
                "build-button-disabled"
            }
        );

        let sell_err = if can_sell {
            html! { <></> }
        } else {
            html! { <p> { "You cannot sell this." } </p> }
        };

        html! {
            <div onclick=click_cb class=style_class>
                <p> { &button_text } </p>
                { cost_display }
                { sell_err }
            </div>
        }
    }

    fn make_change_button(&self, resources: &Resources, x: i32, y: i32, tile: Tile, costs: OwnedResources) -> Html {
        let cost_display = self.make_cost_display(&costs);

        let can_pay = resources.get::<OwnedResources>().unwrap().can_pay(&costs);
        let can_path = resources.get::<Map>().unwrap().can_set_tile(x, y, tile);

        let would_work = can_pay && can_path;

        let click_cb = if would_work {
            self.link.callback(move |_: MouseEvent| DetailViewMsg::ChangeTileButtonClicked {
                x,
                y,
                desired: tile,
                costs: costs.clone(),
            })
        } else {
            self.link.callback(|_: MouseEvent| DetailViewMsg::Nothing)
        };

        let button_text = format!("Change to {:?}", tile);

        let style_class = format!(
            "build-button {}",
            if would_work {
                "build-button-enabled"
            } else {
                "build-button-disabled"
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

    fn make_build_button(
        &self,
        resources: &Resources,
        x: i32,
        y: i32,
        kind: StructureKind,
        costs: OwnedResources,
        structures_present: bool,
    ) -> Html {
        let cost_display = self.make_cost_display(&costs);

        let can_pay = resources.get::<OwnedResources>().unwrap().can_pay(&costs);

        let would_work = can_pay && !structures_present;

        let click_cb = if would_work {
            self.link.callback(move |_: MouseEvent| DetailViewMsg::BuildStructureButtonClicked {
                x,
                y,
                desired: kind,
                costs: costs.clone(),
            })
        } else {
            self.link.callback(move |_: MouseEvent| DetailViewMsg::Nothing)
        };

        let button_text = format!("Build a {:?}", kind);

        let style_class = format!(
            "build-button {}",
            if would_work {
                "build-button-enabled"
            } else {
                "build-button-disabled"
            }
        );

        let pay_err = if can_pay {
            html! {}
        } else {
            html! { <p> { "You cannot afford this." } </p> }
        };

        let blocked_err = if structures_present {
            html! { <p> { "Another structure is already present." } </p> }
        } else {
            html! {}
        };

        html! {
            <div onclick=click_cb class=style_class>
                <p> { &button_text } </p>
                { cost_display }
                { blocked_err }
                { pay_err }
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

    fn tile_details(&self, x: i32, y: i32, tile: Tile, structures: &[StructureState]) -> Html {
        use super::collapsible_div::*;

        let tile_str = format!("Selected tile at ({}, {}): {:?}", x, y, tile);

        let mut sell_structures: Vec<Html> = Vec::new();
        let mut build_structures: Vec<Html> = Vec::new();
        let mut changes: Vec<Html> = Vec::new();

        self.ecs.with(|_, r| {
            for ss in structures {
                sell_structures.push(self.make_structure_view(ss));
            }
            for (target, cost) in r.get::<TileTransforms>().unwrap().list_all_for(tile).into_iter() {
                changes.push(self.make_change_button(r, x, y, target, cost));
            }
            for (kind, cost) in r.get::<StructureBuilds>().unwrap().list_all_for(tile).into_iter() {
                build_structures.push(self.make_build_button(r, x, y, kind, cost, !structures.is_empty()));
            }
        });

        let structures_view = if sell_structures.is_empty() {
            html! {}
        } else {
            html! {
                <Collapsible collapse_name="SellStructures" title="Existing Structures".to_string() ecs=self.ecs.clone()>
                    { sell_structures }
                </Collapsible>
            }
        };

        let build_view = if build_structures.is_empty() {
            html! {}
        } else {
            html! {
                <Collapsible collapse_name="BuildStructures" title="Build Structures".to_string() ecs=self.ecs.clone()>
                    { build_structures }
                </Collapsible>
            }
        };

        let change_tile_view = if changes.is_empty() {
            html! {}
        } else {
            html! {
                <Collapsible collapse_name="TileChanges" title="Tile Changes".to_string() ecs=self.ecs.clone()>
                    { changes }
                </Collapsible>
            }
        };

        html! {
            <div>
                <p>{ tile_str }</p>
                { structures_view }
                { build_view }
                { change_tile_view }
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
            DetailViewMsg::Nothing => {}
            DetailViewMsg::ChangeTileButtonClicked { x, y, desired, costs } => {
                self.ecs.with(|world, _| {
                    world.push((TryChangeTileType { x, y, desired, costs },));
                });
            }
            DetailViewMsg::BuildStructureButtonClicked { x, y, desired, costs } => {
                self.ecs.with(|world, _| {
                    world.push((TryBuildStructure { x, y, desired, costs },));
                });
            }
            DetailViewMsg::SellExistingStructureButtonClicked { to_sell } => {
                self.ecs.with(|world, _| {
                    world.push((TrySellStructure { to_sell },));
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
            DetailState::Tile { x, y, tile, structures } => self.tile_details(*x, *y, *tile, structures),
        }
    }
}

fn empty_pane() -> Html {
    html! {
        <></>
    }
}
