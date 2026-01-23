use yew::prelude::*;

#[function_component(LoginWrapper)]
pub fn login_wrapper(props: &yew::html::ChildrenProps) -> Html {
    let key = nostr_minions::use_nostr_key();

    if key.is_some() {
        html! {
            { props.children.clone() }
        }
    } else {
        html! {
            <super::LoginPage />
        }
    }
}
