use yew::prelude::*;

use crate::{components::*, resources::*, ECS};

pub(crate) struct LaunchWaveView {
    link: ComponentLink<Self>,
    model: ECS,
}

#[derive(Clone, Properties)]
pub(crate) struct LaunchWaveProps {
    pub(crate) ecs: ECS,
}

#[derive(Clone)]
pub(crate) enum LaunchWaveMessage {
    Clicked,
    AutoToggled,
}

impl Component for LaunchWaveView {
    type Message = LaunchWaveMessage;
    type Properties = LaunchWaveProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LaunchWaveView { link, model: props.ecs }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            LaunchWaveMessage::Clicked => {
                self.model.with(|world, _| {
                    world.push((TryLaunchWave,));
                });
            }
            LaunchWaveMessage::AutoToggled => {
                self.model.with(|world, _| {
                    world.push((ToggleAutoLaunchWave,));
                });
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.model = props.ecs;
        true
    }

    fn view(&self) -> Html {
        let next = self.model.with(|_, r| *r.get_or_default::<NextWaveState>());

        let style_class = if next.delay_ticks > 0 {
            "launch-wave-div-disabled "
        } else {
            "launch-wave-div-enabled"
        };
        let style_class = format!("launch-wave-div {}", style_class);

        let click_cb = self.link.callback(|_: MouseEvent| LaunchWaveMessage::Clicked);

        let mut text = format!("Launch wave {}", next.next_wave);

        if next.delay_ticks > 0 {
            text = format!("{} (wait {})", text, next.delay_ticks);
        }

        let img_link = if next.auto_launch {
            "/assets/images/pause-auto-wave.png"
        } else {
            "/assets/images/auto-launch-wave.png"
        };

        let toggle_cb = self.link.callback(|_: MouseEvent| LaunchWaveMessage::AutoToggled);

        html! {
            <div class=style_class onclick=click_cb>
                <img class="vert-center" src=img_link onclick=toggle_cb />
                <div class="vert-center">{ text }</div>
            </div>
        }
    }
}
