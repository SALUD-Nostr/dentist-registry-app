use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let navigate_to_patients = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientsList);
        })
    };

    let navigate_to_calendar = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::EncountersSchedule);
        })
    };

    let navigate_to_new_encounter = {
        let navigator = navigator;
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::EncounterNew);
        })
    };

    html! {
        <div class="flex flex-col gap-4 sm:gap-8  sm:max-w-xl md:max-w-3xl lg:max-w-5xl  size-full sm:items-center sm:justify-center overflow-y-auto flex-1 self-stretch mx-auto px-4 sm:px-6 md:px-8 py-4 sm:py-6 md:py-8">
            <div class="flex flex-col items-center">
                <crate::components::Logo class="size-12 sm:size-16 md:size-20 m-4 text-primary" />
                <h1 class="text-3xl sm:text-4xl md:text-5xl font-bold text-center">
                    {"Salud Dental"}
                </h1>
                <p class="text-center text-muted max-w-lg mt-4">
                    {"Sistema de gestión de pacientes y citas médicas"}
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <HomeCard
                    title="Pacientes"
                    description="Gestiona el registro de pacientes"
                    icon={html!{<crate::components::Users class="size-12 text-primary" />}}
                    onclick={navigate_to_patients}
                />
                <HomeCard
                    title="Calendario"
                    description="Visualiza todas las citas programadas"
                    icon={html!{<crate::components::Calendar class="size-12 text-primary" />}}
                    onclick={navigate_to_calendar}
                />
                <HomeCard
                    title="Nueva Cita"
                    description="Agenda una nueva cita"
                    icon={html!{<crate::components::Plus class="size-12 text-primary" />}}
                    onclick={navigate_to_new_encounter}
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct HomeCardProps {
    title: &'static str,
    description: &'static str,
    icon: Html,
    onclick: Callback<MouseEvent>,
}

#[function_component(HomeCard)]
fn home_card(props: &HomeCardProps) -> Html {
    let HomeCardProps {
        title,
        description,
        icon,
        onclick,
    } = props;

    html! {
        <button
            onclick={onclick}
            class="flex sm:flex-col items-center gap-4 p-6 bg-white rounded-xl border border-muted/30 shadow-md hover:shadow-lg transition-shadow cursor-pointer"
        >
            { icon.clone() }
            <div class="flex flex-col gap-1 sm:gap-2 text-start sm:text-center">
                <h3 class="text-lg font-semibold">
                    { *title }
                </h3>
                <p class="text-sm text-muted">
                    { *description }
                </p>
            </div>
        </button>
    }
}
