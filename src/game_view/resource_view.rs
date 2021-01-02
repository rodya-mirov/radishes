use yew::prelude::*;

use crate::{resources::*, ECS};

pub(crate) struct ResourceView {
    model: ECS,
}

#[derive(Clone, Properties)]
pub(crate) struct ResourcesProps {
    pub(crate) ecs: ECS,
}

#[derive(Clone)]
pub(crate) enum ResourceViewMessage {}

fn resource_view(r: OwnedResource, amt: i64) -> Html {
    let s = format!("{}: {}", r, amt);
    html! {
        <p>{ s }</p>
    }
}

impl Component for ResourceView {
    type Message = ResourceViewMessage;
    type Properties = ResourcesProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // link is not used; no callbacks are needed
        ResourceView { model: props.ecs }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {}
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.model = props.ecs;
        true
    }

    fn view(&self) -> Html {
        let list: Vec<Html> = self.model.with(|_, r| {
            let owned = r.get::<OwnedResources>().unwrap();

            ALL_RESOURCES
                .iter()
                .copied()
                .map(|o| {
                    let amt = owned.0.get(&o).copied().unwrap_or(0);
                    (o, amt)
                })
                .map(|(o, amt)| resource_view(o, amt))
                .collect()
        });

        html! {
            <div>
                { list }
            </div>
        }
    }
}
