use yew::prelude::*;

use crate::{resources::OwnedResources, ECS};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ViewedResources {
    wood: i64,
    metal: i64,
    money: i64,
}

fn from_ecs(ecs: &ECS) -> ViewedResources {
    let (_, r) = &*ecs.lock().unwrap();
    let resources = r.get::<OwnedResources>().unwrap();

    ViewedResources {
        wood: resources.wood,
        metal: resources.metal,
        money: resources.money,
    }
}

pub(crate) struct ResourceView {
    resources: ViewedResources,
}

#[derive(Clone, Properties)]
pub(crate) struct ResourcesProps {
    pub(crate) ecs: ECS,
}

#[derive(Clone)]
pub(crate) enum ResourceViewMessage {}

impl ResourceView {
    fn update_resources(&mut self, ecs: &ECS) -> bool {
        let rv = from_ecs(ecs);

        if rv != self.resources {
            self.resources = rv;
            true
        } else {
            false
        }
    }
}

impl Component for ResourceView {
    type Message = ResourceViewMessage;
    type Properties = ResourcesProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // link is not used; no callbacks are needed
        ResourceView {
            resources: from_ecs(&props.ecs),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {}
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.update_resources(&props.ecs)
    }

    fn view(&self) -> Html {
        html! {
            <div id="resource-view-div">
                <p>{ "Money: " }{ self.resources.money }</p>
                <p>{ "Wood:  "}{ self.resources.wood }</p>
                <p>{ "Metal: " }{ self.resources.metal }</p>
            </div>
        }
    }
}
