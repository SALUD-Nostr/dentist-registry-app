//! Loading spinner component

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoaderProps {
    #[prop_or_default]
    pub class: Classes,
}

/// Loading spinner component
#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    let default_class = classes!("animate-spin", "text-primary");
    let combined_class = classes!(default_class, props.class.clone());

    html! {
        <svg class={combined_class} fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
    }
}
