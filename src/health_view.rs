use yew::prelude::*;

use crate::{resources::*, ECS};

pub(crate) struct HealthView {
    model: ECS,
}

#[derive(Clone, Properties)]
pub(crate) struct HealthProps {
    pub(crate) ecs: ECS,
}

#[derive(Clone)]
pub(crate) enum HealthMsg {}

impl Component for HealthView {
    type Message = HealthMsg;
    type Properties = HealthProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // link is not used; no callbacks are needed
        HealthView { model: props.ecs }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {}
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.model = props.ecs;
        true
    }

    fn view(&self) -> Html {
        let player_health = self.model.with(|_, r| *r.get_or_default::<PlayerHealth>());

        let text = format!("Health: {}/{}", player_health.health, player_health.max);

        html! {
            <div class="health-view">
                { text }
            </div>
        }
    }
}
