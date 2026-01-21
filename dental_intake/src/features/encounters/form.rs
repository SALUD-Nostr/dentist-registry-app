//! Encounter scheduling form

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(EncounterForm)]
pub fn encounter_form() -> Html {
    let navigator = use_navigator().unwrap();

    let handle_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::EncountersSchedule);
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 md:p-8 overflow-auto">
            <div class="max-w-3xl mx-auto w-full">
                <div class="mb-6">
                    <button
                        onclick={handle_cancel.clone()}
                        class="flex items-center gap-2 text-muted hover:text-foreground transition-colors mb-4"
                    >
                        <crate::components::ArrowLeft class="size-5" />
                        {"Volver al Calendario"}
                    </button>
                    <h1 class="text-3xl font-bold">{"Agendar Nueva Cita"}</h1>
                </div>

                <shady_minions::ui::Card>
                    <div class="space-y-6">
                        <div class="text-center py-12">
                            <crate::components::Calendar class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                            <p class="text-lg text-muted-foreground">{"Formulario de agendamiento"}</p>
                            <p class="text-sm mt-2 text-muted">{"Próximamente: Seleccionar paciente, fecha, hora, tipo de cita"}</p>
                        </div>

                        <div class="flex justify-end gap-3">
                            <button
                                onclick={handle_cancel}
                                class="px-4 py-2 border border-muted text-foreground rounded-lg hover:bg-muted/10 transition-colors"
                            >
                                {"Cancelar"}
                            </button>
                            <button
                                disabled={true}
                                class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50"
                            >
                                {"Agendar Cita"}
                            </button>
                        </div>
                    </div>
                </shady_minions::ui::Card>
            </div>
        </div>
    }
}
