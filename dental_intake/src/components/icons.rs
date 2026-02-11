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

/// Settings (cog/gear) icon
#[function_component(Settings)]
pub fn settings(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
    }
}

/// Eye icon (show/reveal)
#[function_component(Eye)]
pub fn eye(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
        </svg>
    }
}

/// Eye Off icon (hide/conceal)
#[function_component(EyeOff)]
pub fn eye_off(props: &IconProps) -> Html {
    html! {
        <svg class={props.class.clone()} fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
        </svg>
    }
}
