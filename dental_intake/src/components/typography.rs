//! Typography components for consistent text styling across the application
//!
//! This module provides standardized typography components to ensure
//! consistent text styling, sizes, and visual hierarchy.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("text-2xl sm:text-3xl font-bold text-foreground".to_string())]
    pub size: String,
}

/// Main title component for headings and important text
#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <h1 class={classes!(props.size.clone(), props.class.clone())}>
            { for props.children.iter() }
        </h1>
    }
}

#[derive(Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("text-lg sm:text-xl font-semibold text-foreground".to_string())]
    pub size: String,
}

/// Subtitle component for secondary headings
#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProps) -> Html {
    html! {
        <h2 class={classes!(props.size.clone(), props.class.clone())}>
            { for props.children.iter() }
        </h2>
    }
}

#[derive(Properties, PartialEq)]
pub struct NormalTextProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("text-base text-foreground".to_string())]
    pub size: String,
}

/// Normal text component for body content and regular paragraphs
#[function_component(NormalText)]
pub fn normal_text(props: &NormalTextProps) -> Html {
    html! {
        <p class={classes!(props.size.clone(), props.class.clone())}>
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct MutedTextProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("text-sm text-muted".to_string())]
    pub size: String,
}

/// Muted text component for secondary information and help text
#[function_component(MutedText)]
pub fn muted_text(props: &MutedTextProps) -> Html {
    html! {
        <p class={classes!(props.size.clone(), props.class.clone())}>
            { for props.children.iter() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or("block text-sm font-medium text-muted mb-1".to_string())]
    pub size: String,
    #[prop_or_default]
    #[prop_or_default]
    pub for_id: Option<String>,
}

/// Label component for form fields and input descriptions
#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    html! {
        <label
            class={classes!(props.size.clone(), props.class.clone())}
            for_id={props.for_id.clone()}
        >
            { for props.children.iter() }
        </label>
    }
}
