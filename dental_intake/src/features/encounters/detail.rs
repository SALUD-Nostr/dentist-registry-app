//! Encounter detail view with clinical impressions

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EncounterDetailProps {
    pub encounter_id: String,
}

#[function_component(EncounterDetail)]
pub fn encounter_detail(props: &EncounterDetailProps) -> Html {
    let navigator = use_navigator().unwrap();

    let handle_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::History);
        })
    };

    let handle_new_impression = {
        let navigator = navigator.clone();
        let encounter_id = props.encounter_id.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::ClinicalImpressionNew {
                encounter_id: encounter_id.clone()
            });
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
                        {"Volver al Historial"}
                    </button>
                    <h1 class="text-3xl font-bold">{"Detalles de la Cita"}</h1>
                    <p class="text-sm text-muted">{"ID: "}{&props.encounter_id}</p>
                </div>

                <div class="grid gap-6">
                    <shady_minions::ui::Card>
                        <h2 class="text-xl font-semibold mb-4">{"Información de la Cita"}</h2>
                        <div class="text-center py-8">
                            <crate::components::Stethoscope class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                            <p class="text-muted">{"Detalles del encuentro - Próximamente"}</p>
                        </div>
                    </shady_minions::ui::Card>

                    <shady_minions::ui::Card>
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold">{"Impresiones Clínicas"}</h2>
                            <button
                                onclick={handle_new_impression}
                                class="px-3 py-1 text-sm bg-primary text-white rounded-lg hover:bg-primary/90 flex items-center gap-1"
                            >
                                <crate::components::Plus class="size-4" />
                                {"Agregar"}
                            </button>
                        </div>
                        <div class="text-center py-8">
                            <crate::components::Clipboard class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                            <p class="text-muted">{"Lista de impresiones clínicas - Próximamente"}</p>
                        </div>
                    </shady_minions::ui::Card>
                </div>
            </div>
        </div>
    }
}
