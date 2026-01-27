//! App logo component

use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct LogoProps {
    #[prop_or_default]
    pub class: Classes,
}

/// App logo - Portal Salud logo
#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    html! {
        <img
            src="/public/img/salud_logo.png"
            alt="Portal Salud"
            class={props.class.clone()}
        />
    }
}
