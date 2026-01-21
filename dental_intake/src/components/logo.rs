//! App logo component

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or_default]
    pub class: Classes,
}

/// App logo - a simple tooth icon
#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    html! {
        <svg class={props.class.clone()} viewBox="0 0 24 24" fill="currentColor">
            // Tooth/dental icon
            <path d="M12 2C9.5 2 7.5 3.5 6.5 5.5C5.5 7.5 5 10 5 12C5 14 5.5 16 6.5 17.5C7 18.5 7.5 19.5 8 20.5C8.5 21.5 9 22 10 22C10.5 22 11 21.5 11 21V19C11 18.5 11 18 11 17.5C11 17 11 16.5 11.5 16C12 15.5 12.5 15.5 13 15.5C13.5 15.5 14 15.5 14.5 16C15 16.5 15 17 15 17.5C15 18 15 18.5 15 19V21C15 21.5 15.5 22 16 22C17 22 17.5 21.5 18 20.5C18.5 19.5 19 18.5 19.5 17.5C20.5 16 21 14 21 12C21 10 20.5 7.5 19.5 5.5C18.5 3.5 16.5 2 14 2C13 2 12.5 2 12 2Z" />
        </svg>
    }
}
