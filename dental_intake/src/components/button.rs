//! Normalized button component with consistent styling across the app

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Destructive,
    Text,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// Button variant (Primary, Secondary, Outline, Destructive, Text)
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,

    /// Button size
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,

    /// Whether the button is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Whether the button is in a loading state
    #[prop_or(false)]
    pub loading: bool,

    /// Full width button
    #[prop_or(false)]
    pub full_width: bool,

    /// Button type (button, submit, reset)
    #[prop_or("button".to_string())]
    pub button_type: String,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Button content (text and/or icons)
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let base_classes = "inline-flex items-center justify-center gap-2 font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:shadow-none";

    // Size classes with consistent text sizing
    let size_classes = match props.size {
        ButtonSize::Small => "px-3 py-1.5 text-sm rounded-md",
        ButtonSize::Medium => "px-4 py-2.5 text-base rounded-lg",
        ButtonSize::Large => "px-6 py-3 text-lg rounded-lg",
    };

    // Variant classes with 3D styling (shadows and subtle gradients)
    let variant_classes = match props.variant {
        ButtonVariant::Primary => {
            "bg-gradient-to-b from-primary to-primary/90 text-white \
             shadow-md shadow-primary/30 \
             hover:shadow-lg hover:shadow-primary/40 hover:-translate-y-0.5 \
             active:translate-y-0 active:shadow-sm \
             focus:ring-primary"
        }
        ButtonVariant::Secondary => {
            "bg-gradient-to-b from-secondary to-secondary/90 text-secondary-foreground \
             shadow-md shadow-secondary/30 \
             hover:shadow-lg hover:shadow-secondary/40 hover:-translate-y-0.5 \
             active:translate-y-0 active:shadow-sm \
             focus:ring-secondary"
        }
        ButtonVariant::Outline => {
            "bg-white border-2 border-muted text-foreground \
             shadow-sm hover:shadow-md hover:border-muted/80 hover:-translate-y-0.5 \
             active:translate-y-0 active:shadow-sm \
             focus:ring-primary"
        }
        ButtonVariant::Destructive => {
            "bg-gradient-to-b from-red-600 to-red-700 text-white \
             shadow-md shadow-red-600/30 \
             hover:shadow-lg hover:shadow-red-600/40 hover:-translate-y-0.5 \
             active:translate-y-0 active:shadow-sm \
             focus:ring-red-600"
        }
        ButtonVariant::Text => {
            "text-primary hover:text-primary/80 hover:underline \
             focus:ring-primary"
        }
    };

    let width_class = if props.full_width { "w-full" } else { "" };

    let classes = classes!(
        base_classes,
        size_classes,
        variant_classes,
        width_class,
        props.class.clone()
    );

    let onclick = props.onclick.clone();
    let disabled = props.disabled || props.loading;

    html! {
        <button
            type={props.button_type.clone()}
            class={classes}
            onclick={onclick}
            disabled={disabled}
        >
            if props.loading {
                <div class="size-4 border-2 border-current border-t-transparent rounded-full animate-spin"></div>
            }
            { for props.children.iter() }
        </button>
    }
}
