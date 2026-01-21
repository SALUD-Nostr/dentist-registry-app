//! Patient registration form

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PatientForm)]
pub fn patient_form() -> Html {
    let navigator = use_navigator().unwrap();

    let handle_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientsList);
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
                        {"Volver a Pacientes"}
                    </button>
                    <h1 class="text-3xl font-bold">{"Registrar Nuevo Paciente"}</h1>
                </div>

                <shady_minions::ui::Card>
                    <div class="space-y-6">
                        <div class="text-center py-12">
                            <crate::components::User class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                            <p class="text-lg text-muted-foreground">{"Formulario de registro de paciente"}</p>
                            <p class="text-sm mt-2 text-muted">{"Próximamente: Campos para nombre, fecha de nacimiento, contacto, dirección"}</p>
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
                                {"Guardar Paciente"}
                            </button>
                        </div>
                    </div>
                </shady_minions::ui::Card>
            </div>
        </div>
    }
}
