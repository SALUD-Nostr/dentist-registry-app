use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarButtonProps<T: Routable + PartialEq + 'static> {
    pub to: T,
    #[prop_or_default]
    pub icon: Html,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(NavbarButton)]
pub fn navbar_button<T: Routable + PartialEq + 'static>(props: &NavbarButtonProps<T>) -> Html {
    let current_route = use_route::<T>();
    let is_active = current_route
        .as_ref()
        .is_some_and(|route| route == &props.to);

    if props.disabled {
        html! {
            <button
                disabled={true}
                class={classes!(
                    "cursor-not-allowed",
                    "p-2",
                    "rounded-md",
                    "transition-all",
                    "duration-300",
                    "whitespace-nowrap",
                    "md:group-hover:w-full",
                    "border",
                    "border-transparent",
                    "opacity-50",
                    props.class.clone(),
                )}
            >
                <NavbarItemContent icon={props.icon.clone()} label={props.label.clone()} />
            </button>
        }
    } else {
        html! {
            <Link<T> to={props.to.clone()}>
                <button class={classes!(
                    "hover:cursor-pointer",
                    "p-2",
                    "rounded-md",
                    "transition-all",
                    "duration-300",
                    "whitespace-nowrap",
                    "md:group-hover:w-full",
                    if is_active { "shadow-xl border border-secondary/30 bg-secondary/30" } else { "border border-transparent" },
                    props.class.clone(),
                )}>
                    <NavbarItemContent icon={props.icon.clone()} label={props.label.clone()} />
                </button>
            </Link<T>>
        }
    }
}

#[function_component(Navbar)]
pub fn navbar(props: &yew::html::ChildrenProps) -> Html {
    html! {
        <>
            // Desktop version
            <nav
                class={classes!(
                    "group",
                    "hidden",
                    "md:block",
                    "md:min-h-full",
                    "bg-background",
                    "overflow-hidden",
                    "px-2",
                    "shadow-xl",
                    "border-r",
                    "border-r-muted/50",
                    "transition-[width]",
                    "duration-300",
                    "ease-in-out",
                    "hover:w-auto",
                    "hover:px-3",
                    "z-[999]",
                )}>
                <div class="flex flex-row md:flex-col gap-1 md:gap-5 overflow-x-auto md:overflow-x-visible py-8">
                    {props.children.clone()}
                </div>
            </nav>

            // Mobile version
            <nav class="md:hidden w-full bg-white border-t border-muted/50 flex items-center justify-around px-2 py-3 relative z-50 mt-2 shadow-lg">
                {props.children.clone()}
            </nav>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct NavbarItemContentProps {
    icon: Html,
    label: String,
    #[prop_or_default]
    text_class: Classes,
}

#[function_component(NavbarItemContent)]
fn navbar_item_content(props: &NavbarItemContentProps) -> Html {
    html! {
        <div class="flex items-center gap-2">
            <div class="shrink-0">
                {props.icon.clone()}
            </div>
            <p class={classes!(
                "text-sm", "transition-all", "duration-300", "overflow-hidden", "whitespace-nowrap",
                "hidden", "sm:block", "text-muted",
                "md:opacity-0", "md:ml-0", "md:max-w-0",
                "md:group-hover:opacity-100", "md:group-hover:ml-1", "md:group-hover:max-w-32",
                props.text_class.clone(),
            )}>
                {&props.label}
            </p>
        </div>
    }
}
