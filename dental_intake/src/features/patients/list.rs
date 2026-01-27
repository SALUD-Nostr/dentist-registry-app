//! Patients list view

use gloo_console::log;
use salud_types::{AdministrativeGender, Patient};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typography::{MutedText, NormalText, Title};
use crate::components::{Button, ButtonSize, ButtonVariant};

#[function_component(PatientsList)]
pub fn patients_list() -> Html {
    let navigator = use_navigator().unwrap();
    let patient_store = crate::storage::use_patient_store();
    let patient_version = crate::storage::use_patient_store_version();

    let search_query = use_state(String::new);
    let patients = use_state(|| Vec::<Patient>::new());
    let filtered_patients = use_state(|| Vec::<Patient>::new());
    let is_loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load patients on mount
    {
        let patients = patients.clone();
        let filtered_patients = filtered_patients.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let patient_store = patient_store.clone();

        use_effect_with(patient_version, move |_version| {
            spawn_local(async move {
                match patient_store.get_all().await {
                    Ok(all_patients) => {
                        log!(
                            "Loaded patients:",
                            format!("{} patients", all_patients.len())
                        );
                        patients.set(all_patients.clone());
                        filtered_patients.set(all_patients);
                        is_loading.set(false);
                    }
                    Err(e) => {
                        log!("Error loading patients:", format!("{:?}", e));
                        error.set(Some(format!("Error al cargar pacientes: {:?}", e)));
                        is_loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Filter patients when search query changes
    {
        let search_query = search_query.clone();
        let patients = patients.clone();
        let filtered_patients = filtered_patients.clone();

        use_effect_with((*search_query).clone(), move |query| {
            let query = query.trim().to_lowercase();

            if query.is_empty() {
                filtered_patients.set((*patients).clone());
            } else {
                let filtered: Vec<Patient> = (*patients)
                    .iter()
                    .filter(|patient| {
                        if let Some(full_name) = patient.full_name() {
                            full_name.to_lowercase().contains(&query)
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();
                filtered_patients.set(filtered);
            }
            || ()
        });
    }

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let handle_new_patient = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientNew);
        })
    };

    let handle_patient_click = {
        let navigator = navigator;
        Callback::from(move |patient_id: String| {
            navigator.push(&crate::router::Route::PatientDetail { id: patient_id });
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            <div class="flex flex-col gap-4 justify-between">
                <div class="flex items-center justify-between gap-4">
                    <Title>{"Pacientes Registrados"}</Title>
                    <div class="text-sm text-muted">
                        {format!("{} paciente(s)", (*filtered_patients).len())}
                    </div>
                </div>
                <div class="flex flex-row items-center gap-2">
                    <input
                        type="text"
                        placeholder="Buscar por nombre..."
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
                    <Button
                        variant={ButtonVariant::Primary}
                        size={ButtonSize::Medium}
                        onclick={Some(handle_new_patient)}
                    >
                        <crate::components::Plus class="size-5" />
                        {"Nuevo Paciente"}
                    </Button>
                </div>
            </div>

            <div class="flex-1 overflow-auto">
                if *is_loading {
                    <div class="flex items-center justify-center h-full">
                        <div class="flex flex-col items-center gap-4">
                            <div class="size-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
                            <MutedText>{"Cargando pacientes..."}</MutedText>
                        </div>
                    </div>
                } else if let Some(err) = (*error).as_ref() {
                    <div class="flex items-center justify-center h-full">
                        <shady_minions::ui::Card class="!border-0 !shadow-none">
                            <div class="flex flex-col items-center gap-4 py-8">
                                <crate::components::X class="size-16 text-red-600" />
                                <NormalText class="text-red-600 font-semibold">{"Error al cargar pacientes"}</NormalText>
                                <MutedText>{err.clone()}</MutedText>
                            </div>
                        </shady_minions::ui::Card>
                    </div>
                } else if (*filtered_patients).is_empty() && (*search_query).is_empty() {
                    // No patients at all
                    <div class="flex items-center justify-center h-full">
                        <shady_minions::ui::Card class="!border-0 !shadow-none">
                            <div class="flex flex-col items-center justify-center gap-4 py-8">
                                <crate::components::Users class="size-16 text-muted opacity-50" />
                                <NormalText class="text-muted text-center font-semibold">
                                    {"No hay pacientes registrados en el sistema."}
                                </NormalText>
                                <MutedText class="text-center">
                                    {"Haz clic en 'Nuevo Paciente' para registrar el primer paciente."}
                                </MutedText>
                            </div>
                        </shady_minions::ui::Card>
                    </div>
                } else if (*filtered_patients).is_empty() {
                    // No results for search
                    <div class="flex items-center justify-center h-full">
                        <shady_minions::ui::Card class="!border-0 !shadow-none">
                            <div class="flex flex-col items-center justify-center gap-4 py-8">
                                <crate::components::Search class="size-16 text-muted opacity-50" />
                                <NormalText class="text-muted text-center font-semibold">
                                    {"No se encontraron pacientes"}
                                </NormalText>
                                <MutedText class="text-center">
                                    {format!("No hay resultados para '{}'", *search_query)}
                                </MutedText>
                            </div>
                        </shady_minions::ui::Card>
                    </div>
                } else {
                    // Display patients table
                    <div class="bg-white rounded-lg shadow-lg border border-muted/30 overflow-hidden">
                        // Header
                        <div class="grid grid-cols-12 gap-2 sm:gap-4 bg-primary/10 px-3 sm:px-6 py-2 sm:py-3 font-semibold text-xs sm:text-sm text-foreground border-b border-muted/30 sticky top-0 z-10">
                            <div class="col-span-6 sm:col-span-4">{"Nombre"}</div>
                            <div class="col-span-3 sm:col-span-2 hidden sm:block">{"Género"}</div>
                            <div class="col-span-3 sm:col-span-3 hidden sm:block">{"Nacimiento"}</div>
                            <div class="col-span-6 sm:col-span-3">{"Teléfono"}</div>
                        </div>

                        // Patient rows
                        <div class="divide-y divide-muted/20">
                            { for (*filtered_patients).iter().map(|patient| {
                                let patient_id = patient.id.clone().unwrap_or_default();
                                let onclick = {
                                    let handle_patient_click = handle_patient_click.clone();
                                    let patient_id = patient_id.clone();
                                    Callback::from(move |_| {
                                        handle_patient_click.emit(patient_id.clone());
                                    })
                                };

                                html! {
                                    <div
                                        key={patient_id.clone()}
                                        onclick={onclick}
                                        class="grid grid-cols-12 gap-2 sm:gap-4 px-3 sm:px-6 py-3 sm:py-4 hover:bg-primary/5 cursor-pointer transition-colors"
                                    >
                                        <div class="col-span-6 sm:col-span-4 flex items-center gap-1 sm:gap-2 min-w-0">
                                            <crate::components::User class="size-4 sm:size-5 text-muted shrink-0" />
                                            <span class="font-medium text-xs sm:text-base text-foreground truncate">
                                                {patient.full_name().unwrap_or_else(|| "Sin nombre".to_string())}
                                            </span>
                                        </div>
                                        <div class="col-span-3 sm:col-span-2 hidden sm:flex items-center text-xs sm:text-sm text-muted truncate">
                                            {
                                                match &patient.gender {
                                                    Some(AdministrativeGender::Male) => "Masculino",
                                                    Some(AdministrativeGender::Female) => "Femenino",
                                                    Some(AdministrativeGender::Other) => "Otro",
                                                    Some(AdministrativeGender::Unknown) | None => "-",
                                                }
                                            }
                                        </div>
                                        <div class="col-span-3 sm:col-span-3 hidden sm:flex items-center text-xs sm:text-sm text-muted truncate">
                                            {
                                                patient.birth_date
                                                    .map(|bd| bd.format("%d/%m/%Y").to_string())
                                                    .unwrap_or_else(|| "-".to_string())
                                            }
                                        </div>
                                        <div class="col-span-6 sm:col-span-3 flex items-center text-xs sm:text-sm text-muted min-w-0">
                                            <span class="truncate">
                                                {
                                                    patient.telecom
                                                        .as_ref()
                                                        .and_then(|telecom| telecom.first())
                                                        .map(|contact| contact.value.clone())
                                                        .unwrap_or_else(|| "-".to_string())
                                                }
                                            </span>
                                        </div>
                                    </div>
                                }
                            }) }
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
