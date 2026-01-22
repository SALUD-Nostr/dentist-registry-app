//! Icon components using SVG

use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct IconProps {
    #[prop_or_default]
    pub class: Classes,
}

/// Home icon
#[function_component(Home)]
pub fn home(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
        </svg>
    }
}

/// Calendar icon
#[function_component(Calendar)]
pub fn calendar(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
    }
}

/// List icon
#[function_component(List)]
pub fn list(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
        </svg>
    }
}

/// Plus icon
#[function_component(Plus)]
pub fn plus(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
    }
}

/// User icon (for patients)
#[function_component(User)]
pub fn user(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
        </svg>
    }
}

/// Users icon (for multiple patients)
#[function_component(Users)]
pub fn users(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
    }
}

/// Clipboard (for clinical impressions)
#[function_component(Clipboard)]
pub fn clipboard(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
        </svg>
    }
}

/// Stethoscope icon (for encounters/medical)
#[function_component(Stethoscope)]
pub fn stethoscope(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19c-4.3 0-8-3.7-8-8V8c0-1.1.9-2 2-2h12c1.1 0 2 .9 2 2v3c0 4.3-3.7 8-8 8zm0 0c0 1.7 1.3 3 3 3h3c1.7 0 3-1.3 3-3m0-5v2c0 1.7-1.3 3-3 3s-3-1.3-3-3" />
        </svg>
    }
}

/// Arrow Left icon
#[function_component(ArrowLeft)]
pub fn arrow_left(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
        </svg>
    }
}

/// Check icon
#[function_component(Check)]
pub fn check(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
    }
}

/// X (close) icon
#[function_component(X)]
pub fn x(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
    }
}

/// Search icon
#[function_component(Search)]
pub fn search(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
    }
}
