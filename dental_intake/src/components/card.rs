//! Card component

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

/// Card container component
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let default_class = classes!("bg-white", "rounded-lg", "shadow-lg", "p-4", "md:p-6");
    let combined_class = classes!(default_class, props.class.clone());

    html! {
        <div class={combined_class}>
            { for props.children.iter() }
        </div>
    }
}
