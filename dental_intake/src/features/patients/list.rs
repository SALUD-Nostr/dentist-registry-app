//! Patients list view

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PatientsList)]
pub fn patients_list() -> Html {
    let navigator = use_navigator().unwrap();
    let search_query = use_state(String::new);

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let handle_new_patient = {
        let navigator = navigator;
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientNew);
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            <div class="flex flex-col gap-4 justify-between">
                <div class="flex items-center justify-between gap-4">
                    <h2 class="text-2xl font-bold text-foreground">{"Pacientes Registrados"}</h2>
                </div>
                <div class="flex flex-row items-center gap-2">
                    <input
                        type="text"
                        placeholder="Buscar por nombre o identificador..."
                        value={(*search_query).clone()}
                        oninput={on_search_input}
                        class={classes!(
                            "flex-1",
                            "border",
                            "border-muted",
                            "text-foreground",
                            "text-sm",
                            "sm:text-base",
                            "rounded-lg",
                            "py-2",
                            "px-4",
                            "shadow-sm",
                            "focus:outline-none",
                            "focus:border-primary",
                            "focus:ring-1",
                            "focus:ring-primary",
                            "transition-all",
                            "placeholder:text-muted",
                            "duration-150",
                        )}
                    />
                    <button
                        onclick={handle_new_patient}
                        class={classes!(
                            "px-4",
                            "py-2",
                            "bg-primary",
                            "text-white",
                            "rounded-lg",
                            "hover:bg-primary/90",
                            "transition-colors",
                            "text-sm",
                            "font-medium",
                            "whitespace-nowrap",
                        )}
                    >
                        {"+ Nuevo Paciente"}
                    </button>
                </div>
            </div>

            <div class="flex-1">
                <div class="bg-white rounded-lg shadow-md border border-muted/30 overflow-hidden">
                    // Header
                    <div class="grid grid-cols-12 gap-4 bg-primary/10 px-6 py-3 font-semibold text-sm text-foreground border-b border-muted/30 sticky top-0 z-10">
                        <div class="col-span-4">{"Nombre"}</div>
                        <div class="col-span-2">{"Género"}</div>
                        <div class="col-span-3">{"Fecha de Nacimiento"}</div>
                        <div class="col-span-3">{"Teléfono"}</div>
                    </div>

                    // Placeholder content
                    <shady_minions::ui::Card class="!size-fit !flex-grow-0 m-4">
                        <div class="flex flex-col items-center justify-center gap-4 py-8">
                            <crate::components::Users class="size-16 text-muted opacity-50" />
                            <p class="text-muted text-center">
                                {"No hay pacientes registrados en el sistema."}
                            </p>
                            <p class="text-sm text-muted text-center">
                                {"Haz clic en 'Nuevo Paciente' para registrar el primer paciente."}
                            </p>
                        </div>
                    </shady_minions::ui::Card>
                </div>
            </div>
        </div>
    }
}
