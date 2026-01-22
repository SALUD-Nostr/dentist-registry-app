//! Patient detail view

use gloo_console::log;
use salud_types::{AdministrativeGender, ContactPointSystem, Patient};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct PatientDetailProps {
    pub patient_id: String,
}

#[function_component(PatientDetail)]
pub fn patient_detail(props: &PatientDetailProps) -> Html {
    let navigator = use_navigator().unwrap();
    let patient_store = crate::storage::use_patient_store();

    let patient = use_state(|| None::<Patient>);
    let is_loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load patient on mount
    {
        let patient_id = props.patient_id.clone();
        let patient = patient.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let patient_store = patient_store.clone();

        use_effect_with(patient_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                log!("Loading patient:", id.as_str());
                match patient_store.get(&id).await {
                    Ok(Some(p)) => {
                        log!("Patient loaded successfully");
                        patient.set(Some(p));
                        is_loading.set(false);
                    }
                    Ok(None) => {
                        log!("Patient not found");
                        error.set(Some("Paciente no encontrado".to_string()));
                        is_loading.set(false);
                    }
                    Err(e) => {
                        log!("Error loading patient:", format!("{:?}", e));
                        error.set(Some(format!("Error al cargar paciente: {:?}", e)));
                        is_loading.set(false);
                    }
                }
            });
            || ()
        });
    }

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
                        onclick={handle_back.clone()}
                        class="flex items-center gap-2 text-muted hover:text-foreground transition-colors mb-4"
                    >
                        <crate::components::ArrowLeft class="size-5" />
                        {"Volver a Pacientes"}
                    </button>

                    if let Some(p) = (*patient).as_ref() {
                        <h1 class="text-3xl font-bold">
                            {p.full_name().unwrap_or_else(|| "Sin nombre".to_string())}
                        </h1>
                        <p class="text-sm text-muted">{"ID: "}{&props.patient_id}</p>
                    } else {
                        <h1 class="text-3xl font-bold">{"Detalles del Paciente"}</h1>
                        <p class="text-sm text-muted">{"ID: "}{&props.patient_id}</p>
                    }
                </div>

                if *is_loading {
                    <div class="flex items-center justify-center py-12">
                        <div class="flex flex-col items-center gap-4">
                            <div class="size-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
                            <p class="text-muted">{"Cargando información del paciente..."}</p>
                        </div>
                    </div>
                } else if let Some(err) = (*error).as_ref() {
                    <shady_minions::ui::Card>
                        <div class="flex flex-col items-center gap-4 py-12">
                            <crate::components::X class="size-16 text-red-600" />
                            <p class="text-red-600 font-semibold text-lg">{err}</p>
                            <button
                                onclick={handle_back.clone()}
                                class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors"
                            >
                                {"Volver a la lista"}
                            </button>
                        </div>
                    </shady_minions::ui::Card>
                } else if let Some(p) = (*patient).as_ref() {
                    <div class="grid gap-6">
                        // Personal Information Card
                        <shady_minions::ui::Card>
                            <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                                <crate::components::User class="size-6 text-primary" />
                                {"Información Personal"}
                            </h2>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                // Full Name
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Nombre Completo"}
                                    </label>
                                    <p class="text-base font-medium text-foreground">
                                        {p.full_name().unwrap_or_else(|| "-".to_string())}
                                    </p>
                                </div>

                                // Gender
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Género"}
                                    </label>
                                    <p class="text-base text-foreground">
                                        {match &p.gender {
                                            Some(AdministrativeGender::Male) => "Masculino",
                                            Some(AdministrativeGender::Female) => "Femenino",
                                            Some(AdministrativeGender::Other) => "Otro",
                                            Some(AdministrativeGender::Unknown) | None => "-",
                                        }}
                                    </p>
                                </div>

                                // Birth Date
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Fecha de Nacimiento"}
                                    </label>
                                    <p class="text-base text-foreground">
                                        {p.birth_date
                                            .map(|bd| bd.format("%d/%m/%Y").to_string())
                                            .unwrap_or_else(|| "-".to_string())}
                                    </p>
                                </div>

                                // Age (calculated if birth date exists)
                                {
                                    p.birth_date.map(|birth_date| {
                                        let today = chrono::Local::now().date_naive();
                                        let age = today.years_since(birth_date).unwrap_or(0);
                                        html! {
                                            <div>
                                                <label class="block text-sm font-medium text-muted mb-1">
                                                    {"Edad"}
                                                </label>
                                                <p class="text-base text-foreground">
                                                    {format!("{} años", age)}
                                                </p>
                                            </div>
                                        }
                                    })
                                }

                                // Active Status
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Estado"}
                                    </label>
                                    <p class="text-base">
                                        if p.active.unwrap_or(false) {
                                            <span class="inline-flex items-center gap-1 px-2 py-1 bg-green-100 text-green-800 rounded-md text-sm font-medium">
                                                <crate::components::Check class="size-4" />
                                                {"Activo"}
                                            </span>
                                        } else {
                                            <span class="inline-flex items-center gap-1 px-2 py-1 bg-gray-100 text-gray-800 rounded-md text-sm font-medium">
                                                {"Inactivo"}
                                            </span>
                                        }
                                    </p>
                                </div>
                            </div>
                        </shady_minions::ui::Card>

                        // Contact Information Card
                        if p.telecom.is_some() && !p.telecom.as_ref().unwrap().is_empty() {
                            <shady_minions::ui::Card>
                                <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                                    <svg class="size-6 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                                    </svg>
                                    {"Información de Contacto"}
                                </h2>
                                <div class="space-y-4">
                                    { for p.telecom.as_ref().unwrap().iter().map(|contact| {
                                        html! {
                                            <div class="flex items-start gap-3 p-3 bg-gray-50 rounded-lg">
                                                <div class="mt-0.5">
                                                    {match contact.system {
                                                        ContactPointSystem::Phone => html! {
                                                            <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                                                            </svg>
                                                        },
                                                        ContactPointSystem::Email => html! {
                                                            <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                                                            </svg>
                                                        },
                                                        ContactPointSystem::Fax => html! {
                                                            <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />
                                                            </svg>
                                                        },
                                                        ContactPointSystem::Sms => html! {
                                                            <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                                                            </svg>
                                                        },
                                                    }}
                                                </div>
                                                <div class="flex-1">
                                                    <p class="text-sm font-medium text-muted mb-1">
                                                        {match contact.system {
                                                            ContactPointSystem::Phone => "Teléfono",
                                                            ContactPointSystem::Email => "Email",
                                                            ContactPointSystem::Fax => "Fax",
                                                            ContactPointSystem::Sms => "SMS",
                                                        }}
                                                        {
                                                            if let Some(use_) = &contact.use_ {
                                                                format!(" ({})", match use_ {
                                                                    salud_types::ContactPointUse::Home => "Casa",
                                                                    salud_types::ContactPointUse::Work => "Trabajo",
                                                                    salud_types::ContactPointUse::Mobile => "Móvil",
                                                                })
                                                            } else {
                                                                String::new()
                                                            }
                                                        }
                                                    </p>
                                                    <p class="text-base font-medium text-foreground">
                                                        {&contact.value}
                                                    </p>
                                                </div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            </shady_minions::ui::Card>
                        }

                        // Address Information Card
                        if p.address.is_some() && !p.address.as_ref().unwrap().is_empty() {
                            <shady_minions::ui::Card>
                                <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                                    <svg class="size-6 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                                    </svg>
                                    {"Dirección"}
                                </h2>
                                <div class="space-y-4">
                                    { for p.address.as_ref().unwrap().iter().map(|addr| {
                                        html! {
                                            <div class="p-4 bg-gray-50 rounded-lg">
                                                // Full text address if available
                                                if let Some(text) = &addr.text {
                                                    <p class="text-base font-medium text-foreground mb-3">
                                                        {text}
                                                    </p>
                                                }

                                                // Structured address
                                                <div class="space-y-2 text-sm">
                                                    if let Some(lines) = &addr.line {
                                                        { for lines.iter().map(|line| html! {
                                                            <p class="text-muted">{line}</p>
                                                        }) }
                                                    }

                                                    <div class="flex flex-wrap gap-x-4 gap-y-1">
                                                        if let Some(city) = &addr.city {
                                                            <span class="text-foreground font-medium">{city}</span>
                                                        }
                                                        if let Some(state) = &addr.state {
                                                            <span class="text-muted">{state}</span>
                                                        }
                                                        if let Some(postal_code) = &addr.postal_code {
                                                            <span class="text-muted">{postal_code}</span>
                                                        }
                                                    </div>

                                                    if let Some(country) = &addr.country {
                                                        <p class="text-muted">{country}</p>
                                                    }
                                                </div>
                                            </div>
                                        }
                                    }) }
                                </div>
                            </shady_minions::ui::Card>
                        }
                    </div>
                }
            </div>
        </div>
    }
}
