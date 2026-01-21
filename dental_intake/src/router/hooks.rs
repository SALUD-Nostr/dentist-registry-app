use yew::prelude::*;

/// Hook that returns true only when the specified route is currently active/visible
/// This prevents data loading for prerendered but offscreen pages
#[hook]
pub fn use_is_route_active(target_route: super::Route) -> bool {
    let route = yew_router::hooks::use_route::<super::Route>();
    route == Some(target_route)
}

/// Hook that returns true only when ANY of the specified routes matches the predicate
#[hook]
pub fn use_is_any_route_active(check: impl Fn(&super::Route) -> bool + 'static) -> bool {
    let route = yew_router::hooks::use_route::<super::Route>();
    route.as_ref().is_some_and(check)
}
