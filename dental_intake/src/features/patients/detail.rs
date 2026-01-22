//! Patient detail view

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct PatientDetailProps {
    pub patient_id: String,
}

#[function_component(PatientDetail)]
pub fn patient_detail(props: &PatientDetailProps) -> Html {
    let navigator = use_navigator().unwrap();

    let handle_back = {
        let navigator = navigator;
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientsList);
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 md:p-8 overflow-auto">
            <div class="max-w-4xl mx-auto w-full">
                <div class="mb-6">
                    <button
                        onclick={handle_back}
                        class="flex items-center gap-2 text-muted hover:text-foreground transition-colors mb-4"
                    >
                        <crate::components::ArrowLeft class="size-5" />
                        {"Volver a Pacientes"}
                    </button>
                    <h1 class="text-3xl font-bold">{"Detalles del Paciente"}</h1>
                    <p class="text-sm text-muted">{"ID: "}{&props.patient_id}</p>
                </div>

                <div class="grid gap-6">
                    <shady_minions::ui::Card>
                        <h2 class="text-xl font-semibold mb-4">{"Información Personal"}</h2>
                        <div class="text-center py-8">
                            <crate::components::User class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                            <p class="text-muted">{"Información del paciente - Próximamente"}</p>
                        </div>
                    </shady_minions::ui::Card>

                    <shady_minions::ui::Card>
                        <h2 class="text-xl font-semibold mb-4">{"Historial de Encuentros"}</h2>
                        <div class="text-center py-8">
                            <crate::components::Calendar class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                            <p class="text-muted">{"Lista de encuentros médicos - Próximamente"}</p>
                        </div>
                    </shady_minions::ui::Card>
                </div>
            </div>
        </div>
    }
}
