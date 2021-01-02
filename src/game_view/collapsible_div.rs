use yew::prelude::*;

use crate::resources::MenuCollapseStates;
use crate::ECS;

pub struct Collapsible {
    ecs: ECS,
    collapse_name: String,
    title: String,
    children: Children,

    link: ComponentLink<Self>,
}

pub enum Message {
    ToggleCollapse,
}

#[derive(Properties, Clone)]
pub struct CollapsibleProperties {
    pub ecs: ECS,
    pub collapse_name: String,
    pub title: String,
    pub children: Children,
}

impl Component for Collapsible {
    type Message = Message;
    type Properties = CollapsibleProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ecs: props.ecs,
            collapse_name: props.collapse_name,
            title: props.title,
            children: props.children,

            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleCollapse => self.ecs.with(|_, r| {
                let collapse_states: &mut MenuCollapseStates = &mut r.get_mut::<MenuCollapseStates>().unwrap();
                let is_collapsed = collapse_states.is_collapsed(&self.collapse_name);
                collapse_states.set_collapsed(self.collapse_name.clone(), !is_collapsed);
            }),
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.ecs = props.ecs;
        self.collapse_name = props.collapse_name;
        self.title = props.title;
        self.children = props.children;

        true
    }

    fn view(&self) -> Html {
        use yew::html::ChildrenRenderer;

        let is_collapsed = self.ecs.with(|_, r| {
            let collapse_states = r.get::<MenuCollapseStates>().unwrap();
            let is_collapsed = collapse_states.is_collapsed(&self.collapse_name);
            is_collapsed
        });

        html! {
            <div class="collapsible-div">
                <div class="collapsible-title-row">
                    { self.make_collapse_button(is_collapsed) }
                    <div class="collapsible-title"> { &self.title }</div>
                </div>
                { if !is_collapsed { self.children.clone() } else { ChildrenRenderer::new(vec![]) } }
            </div>
        }
    }
}

impl Collapsible {
    fn make_collapse_button(&self, is_collapsed: bool) -> Html {
        let cb = self.link.callback(|_: MouseEvent| Message::ToggleCollapse);

        let text = if is_collapsed { "[+]" } else { "[-]" };

        let class = if is_collapsed {
            "collapse-button collapse-button-collapsed"
        } else {
            "collapse-button collapse-button-expanded"
        };

        // TODO styling
        html! {
            <div class=class onclick=cb>{text}</div>
        }
    }
}
