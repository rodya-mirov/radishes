use web_sys::MouseEvent;
use yew::prelude::*;

use crate::{resources::*, ECS};

pub(crate) struct NewGameView {
    model: ECS,
}

#[derive(Clone, Properties)]
pub(crate) struct EcsProps {
    pub(crate) ecs: ECS,
}

#[derive(Copy, Clone)]
pub(crate) enum NoMsg {}

#[derive(Copy, Clone)]
enum ClickMsg {
    Clicked,
}

impl Component for NewGameView {
    type Message = NoMsg;
    type Properties = EcsProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NewGameView { model: props.ecs }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {}
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.model = props.ecs;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="new-game-menu">
                <div><p>{ "Radishes Have Their Own Value" }</p></div>
                <StartGameBtn ecs=self.model.clone() />
            </div>
        }
    }
}

struct StartGameBtn {
    link: ComponentLink<Self>,
    model: ECS,
}

impl Component for StartGameBtn {
    type Message = ClickMsg;
    type Properties = EcsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, model: props.ecs }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            ClickMsg::Clicked => {
                self.model.with(|_, r| {
                    *r.get_mut::<GameState>().unwrap() = GameState::MainGame;
                });
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.model = props.ecs;
        true
    }

    fn view(&self) -> Html {
        let click_cb = self.link.callback(|_: MouseEvent| ClickMsg::Clicked);

        html! {
            <div class="new-game-button" onclick=click_cb>
                { "Start Game" }
            </div>
        }
    }
}
