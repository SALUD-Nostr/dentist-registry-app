mod hooks;
mod navbar;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/patients")]
    PatientsList,
    #[at("/patients/new")]
    PatientNew,
    #[at("/patients/:id")]
    PatientDetail { id: String },
    #[at("/encounters")]
    EncountersSchedule,
    #[at("/encounters/new")]
    EncounterNew,
    #[at("/encounters/:id")]
    EncounterDetail { id: String },
    #[at("/encounters/:encounter_id/impression/new")]
    ClinicalImpressionNew { encounter_id: String },
    #[at("/history")]
    History,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Keep AppRoute as alias for backward compatibility
pub type AppRoute = Route;

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    // TODO: Re-enable sync status check after Nostr integration restored
    // let sync_status = crate::features::nostr_notes::use_sync_status();
    // let is_synced = sync_status
    //     .as_ref()
    //     .is_some_and(|s| **s == crate::features::nostr_notes::SyncStatus::Synced);

    html! {
        <div class="flex h-[100vh] w-[100vw] flex-col-reverse md:flex-row">
            <navbar::Navbar >
                <crate::components::Logo class="size-8 mx-auto hidden md:block" />
                <div class="w-full h-px bg-muted/50 my-2 hidden md:block" />
                <navbar::NavbarButton<AppRoute> to={AppRoute::Home} icon={html!{<crate::components::Home class="size-6 text-secondary" />}} label={"Inicio"} />
                <navbar::NavbarButton<AppRoute> to={AppRoute::PatientsList} icon={html!{<crate::components::Users class="size-6 text-secondary" />}} label={"Pacientes"} />
                <navbar::NavbarButton<AppRoute> to={AppRoute::EncountersSchedule} icon={html!{<crate::components::Calendar class="size-6 text-secondary" />}} label={"Citas"} />
                <navbar::NavbarButton<AppRoute> to={AppRoute::History} icon={html!{<crate::components::List class="size-6 text-secondary" />}} label={"Historial"} />
            </navbar::Navbar>
            <div class="self-stretch relative flex flex-1 overflow-hidden">
                <AppSwitch />
            </div>
        </div>
    }
}

#[function_component(AppSwitch)]
fn app_switch() -> Html {
    let route = use_route::<AppRoute>();
    let base_class = classes!(
        "self-stretch",
        "size-full",
        "transition-all",
        "duration-300",
        "ease-in-out",
    );
    let visible_class = classes!("translate-x-0", "pointer-events-auto",);
    let invisible_class = classes!("-translate-x-[400%]", "absolute", "pointer-events-none",);

    let loader = html! {
        <div class="flex h-full w-full items-center justify-center">
            <crate::components::Logo class="size-12 animate-pulse" />
        </div>
    };

    html! {
        <>
            // Home/Dashboard
            <div class={classes!(base_class.clone(), if route == Some(AppRoute::Home) { visible_class.clone() } else { invisible_class.clone() })}>
                <crate::features::home::Home />
            </div>

            // Patients List
            <div class={classes!(base_class.clone(), if route == Some(AppRoute::PatientsList) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                    <crate::features::patients::PatientsList />
                </yew::suspense::Suspense>
            </div>

            // Patient New
            <div class={classes!(base_class.clone(), if route == Some(AppRoute::PatientNew) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                    <crate::features::patients::PatientForm />
                </yew::suspense::Suspense>
            </div>

            // Patient Detail
            <div class={classes!(base_class.clone(), if matches!(route, Some(AppRoute::PatientDetail{..})) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                {
                    if let Some(AppRoute::PatientDetail { ref id }) = route {
                        html! { <crate::features::patients::PatientDetail patient_id={id.clone()} /> }
                    } else {
                        html! {}
                    }
                }
                </yew::suspense::Suspense>
            </div>

            // Encounters Schedule (Calendar)
            <div class={classes!(base_class.clone(), if route == Some(AppRoute::EncountersSchedule) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                    <crate::features::encounters::EncountersSchedule />
                </yew::suspense::Suspense>
            </div>

            // Encounter New
            <div class={classes!(base_class.clone(), if route == Some(AppRoute::EncounterNew) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                    <crate::features::encounters::EncounterForm />
                </yew::suspense::Suspense>
            </div>

            // Encounter Detail
            <div class={classes!(base_class.clone(), if matches!(route, Some(AppRoute::EncounterDetail{..})) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                {
                    if let Some(AppRoute::EncounterDetail { ref id }) = route {
                        html! { <crate::features::encounters::EncounterDetail encounter_id={id.clone()} /> }
                    } else {
                        html! {}
                    }
                }
                </yew::suspense::Suspense>
            </div>

            // Clinical Impression New
            <div class={classes!(base_class.clone(), if matches!(route, Some(AppRoute::ClinicalImpressionNew{..})) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                {
                    if let Some(AppRoute::ClinicalImpressionNew { ref encounter_id }) = route {
                        html! { <crate::features::clinical_impressions::ClinicalImpressionForm encounter_id={encounter_id.clone()} /> }
                    } else {
                        html! {}
                    }
                }
                </yew::suspense::Suspense>
            </div>

            // History
            <div class={classes!(base_class.clone(), if route == Some(AppRoute::History) { visible_class.clone() } else { invisible_class.clone() })}>
                <yew::suspense::Suspense fallback={loader.clone()}>
                    <crate::features::encounters::EncountersHistory />
                </yew::suspense::Suspense>
            </div>
        </>
    }
}
