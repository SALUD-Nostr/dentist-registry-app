//! Patient detail view

use chrono::Datelike;
use gloo_console::log;
use salud_types::{
    AdministrativeGender, ContactPointSystem, Encounter, EncounterClass, EncounterStatus, Patient,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typography::{Label, MutedText, NormalText, Subtitle, Title};

#[derive(Properties, PartialEq, Eq)]
pub struct PatientDetailProps {
    pub patient_id: String,
}

#[function_component(PatientDetail)]
pub fn patient_detail(props: &PatientDetailProps) -> Html {
    let navigator = use_navigator().unwrap();
    let patient_store = crate::storage::use_patient_store();
    let encounter_store = crate::storage::use_encounter_store();

    let patient = use_state(|| None::<Patient>);
    let encounters = use_state(Vec::<Encounter>::new);
    let sorted_encounters = {
        let mut encs = (*encounters).clone();
        encs.sort_by(|a, b| {
            let date_a = a
                .period
                .as_ref()
                .and_then(|p| p.start)
                .unwrap_or_else(chrono::Utc::now);
            let date_b = b
                .period
                .as_ref()
                .and_then(|p| p.start)
                .unwrap_or_else(chrono::Utc::now);
            date_b.cmp(&date_a)
        });
        encs
    };
    let is_loading = use_state(|| true);
    let is_loading_encounters = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load patient on mount
    {
        let patient = patient.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();

        use_effect_with(props.patient_id.clone(), move |id| {
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
                        error.set(Some(format!("Error al cargar paciente: {e:?}",)));
                        is_loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Load encounters for this patient on mount
    {
        let patient_id = props.patient_id.clone();
        let encounters = encounters.clone();
        let is_loading_encounters = is_loading_encounters.clone();
        let encounter_store = encounter_store.clone();

        use_effect_with(patient_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                log!("Loading encounters for patient:", id.as_str());
                match encounter_store.get_by_patient(&id).await {
                    Ok(encs) => {
                        log!("Encounters loaded:", format!("{} encounters", encs.len()));
                        encounters.set(encs);
                        is_loading_encounters.set(false);
                    }
                    Err(e) => {
                        log!("Error loading encounters:", format!("{:?}", e));
                        is_loading_encounters.set(false);
                    }
                }
            });
            || ()
        });
    }

    let handle_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientsList);
        })
    };

    html! {
        <div class="flex flex-col size-full detail-page overflow-auto">
            <div class="max-w-4xl mx-auto w-full">
                <div class="mb-4">
                    <button
                        onclick={handle_back.clone()}
                        class="flex items-center gap-2 text-muted hover:text-foreground transition-colors mb-3"
                    >
                        <crate::components::ArrowLeft class="size-4" />
                        {"Volver a Pacientes"}
                    </button>

                    if let Some(p) = (*patient).as_ref() {
                        <Title>
                            {p.full_name().unwrap_or_else(|| "Sin nombre".to_string())}
                        </Title>
                        <MutedText class="text-xs">{"ID: "}{&props.patient_id}</MutedText>
                    } else {
                        <Title>{"Detalles del Paciente"}</Title>
                        <MutedText class="text-xs">{"ID: "}{&props.patient_id}</MutedText>
                    }
                </div>

                if *is_loading {
                    <div class="flex items-center justify-center py-8">
                        <div class="flex flex-col items-center gap-4">
                            <div class="size-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
                            <MutedText>{"Cargando información del paciente..."}</MutedText>
                        </div>
                    </div>
                } else if let Some(err) = (*error).as_ref() {
                    <shady_minions::ui::Card class="border-muted/30 shadow-lg">
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
                    <div class="grid gap-4">
                        // Two column layout for patient info and contact
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            // Personal Information Card
                            <shady_minions::ui::Card class="border-muted/30 shadow-lg">
                                <div class="p-6">
                                    <div class="mb-4 flex items-center gap-2 justify-between">
                                        <Subtitle>{"Información Personal"}</Subtitle>
                                        <NormalText class="text-base">
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
                                        </NormalText>
                                    </div>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <Label class="block mb-1">{"Nombre Completo"}</Label>
                                            <NormalText class="font-medium">
                                                {p.full_name().unwrap_or_else(|| "-".to_string())}
                                            </NormalText>
                                        </div>
                                        <div>
                                            <Label class="block mb-1">{"Género"}</Label>
                                            <NormalText>
                                                {match &p.gender {
                                                    Some(AdministrativeGender::Male) => "Masculino",
                                                    Some(AdministrativeGender::Female) => "Femenino",
                                                    Some(AdministrativeGender::Other) => "Otro",
                                                    Some(AdministrativeGender::Unknown) | None => "-",
                                                }}
                                            </NormalText>
                                        </div>
                                        <div>
                                            <Label class="block mb-1">{"Fecha de Nacimiento"}</Label>
                                            <NormalText>
                                                {p.birth_date
                                                    .map(|bd| bd.format("%d/%m/%Y").to_string())
                                                    .unwrap_or_else(|| "-".to_string())}
                                            </NormalText>
                                        </div>
                                        <div>
                                            <Label class="block mb-1">{"Edad"}</Label>
                                            <NormalText>
                                                {p.birth_date.map(|bd| format!("{} años", chrono::Local::now().date_naive().years_since(bd).unwrap_or(0))).unwrap_or_else(|| "-".to_string())}
                                            </NormalText>
                                        </div>
                                    </div>
                                </div>
                            </shady_minions::ui::Card>

                            // Contact Information Card
                            if p.telecom.is_some() && !p.telecom.as_ref().unwrap().is_empty() {
                                <shady_minions::ui::Card class="border-muted/30 shadow-lg">
                                    <div class="p-6">
                                        <Subtitle class="mb-4 flex items-center gap-2">
                                            <svg class="size-6 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                                            </svg>
                                            {"Información de Contacto"}
                                        </Subtitle>
                                        <div class="grid grid-cols-1 grid-cols-2 gap-4">
                                            { for p.telecom.as_ref().unwrap().iter().map(|contact| {
                                                html! {
                                                    <div class="flex items-start gap-3 p-3 bg-gray-50 rounded-lg">
                                                        <div class="mt-0.5">
                                                            {match contact.system {
                                                                ContactPointSystem::Phone => html! {
                                                                    <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                                                                    </svg>
                                                                },
                                                                ContactPointSystem::Email => html! {
                                                                    <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2h-1H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                                                                    </svg>
                                                                },
                                                                ContactPointSystem::Fax => html! {
                                                                    <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1 1v14a1 1 0 001-1v10a2 2 0 002 2z" />
                                                                    </svg>
                                                                },
                                                                ContactPointSystem::Sms => html! {
                                                                    <svg class="size-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 012 2h-5l-5 5v-5z" />
                                                                    </svg>
                                                                },
                                                            }}
                                                        </div>
                                                        <div class="flex-1">
                                                            <Label class="font-medium mb-1">
                                                                {match contact.system {
                                                                    ContactPointSystem::Phone => "Teléfono",
                                                                    ContactPointSystem::Email => "Email",
                                                                    ContactPointSystem::Fax => "Fax",
                                                                    ContactPointSystem::Sms => "SMS",
                                                                }}
                                                            </Label>
                                                            <NormalText class="font-medium">
                                                                {contact.value.clone()}
                                                            </NormalText>
                                                        </div>
                                                    </div>
                                                }
                                            }) }
                                        </div>
                                    </div>
                                </shady_minions::ui::Card>
                            }
                        </div>

                        // Address Information Card
                        if p.address.is_some() && !p.address.as_ref().unwrap().is_empty() {
                            <shady_minions::ui::Card class="border-muted/30 shadow-lg">
                                <div class="p-6">
                                    <Subtitle class="mb-4 flex items-center gap-2">
                                        <svg class="size-6 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                                        </svg>
                                        {"Dirección"}
                                    </Subtitle>
                                    <div class="space-y-4">
                                        { for p.address.as_ref().unwrap().iter().map(|addr| {
                                            html! {
                                                <div class="p-4 bg-gray-50 rounded-lg">
                                                    // Full text address if available
                                                    if let Some(text) = &addr.text {
                                                        <NormalText class="font-medium mb-3">
                                                            {text.clone()}
                                                        </NormalText>
                                                    }

                                                    // Structured address
                                                    <div class="space-y-2 text-sm">
                                                        if let Some(lines) = &addr.line {
                                                            { for lines.iter().map(|line| html! {
                                                                <MutedText class="text-sm">{line.clone()}</MutedText>
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
                                                            <MutedText class="text-sm">{country.clone()}</MutedText>
                                                        }
                                                    </div>
                                                </div>
                                            }
                                        }) }
                                    </div>
                                </div>
                            </shady_minions::ui::Card>
                        }

                        // Encounters Card
                        <shady_minions::ui::Card class="!border-0 !shadow-none">
                                <Subtitle class="mb-4 flex items-center gap-2">
                                    <crate::components::Stethoscope class="size-6 text-primary" />
                                    {"Citas del Paciente"}
                                </Subtitle>

                                if *is_loading_encounters {
                                    <div class="flex items-center justify-center py-8">
                                        <div class="size-8 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
                                    </div>
                                } else if (*encounters).is_empty() {
                                    <div class="flex flex-col items-center justify-center gap-4 py-8">
                                        <crate::components::Calendar class="size-16 text-muted opacity-50" />
                                        <MutedText class="text-center font-semibold">
                                            {"No hay citas registradas"}
                                        </MutedText>
                                        <MutedText class="text-sm text-center">
                                            {"Las citas del paciente aparecerán aquí."}
                                        </MutedText>
                                    </div>
                                } else {
                                    <div class="space-y-3">
                                        { for sorted_encounters.iter().map(|encounter| {
                                            html! { <EncounterPreviewCard encounter={encounter.clone()} /> }
                                        }) }
                                    </div>
                                }
                        </shady_minions::ui::Card>
                    </div>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct EncounterPreviewProps {
    pub encounter: Encounter,
}

#[function_component(EncounterPreviewCard)]
fn encounter_preview_card(props: &EncounterPreviewProps) -> Html {
    let EncounterPreviewProps { encounter } = props;
    let Some(date) = encounter.period.as_ref().and_then(|p| p.start) else {
        return html! {};
    };
    let Some(encounter_id) = encounter.id.clone() else {
        return html! {};
    };
    html! {
        <yew_router::components::Link::<crate::router::Route>
            to={crate::router::AppRoute::EncounterDetail { id: encounter_id }}
            classes={classes!("flex", "flex-col", "sm:flex-row", "sm:items-center", "justify-between", "gap-4", "p-4", "bg-gray-50", "rounded-lg", "hover:bg-gray-100", "transition-colors", "cursor-pointer", "border", "border-muted/30", "shadow-lg", "hover:border-l-4", "hover:border-primary")}
        >
            <div class="flex items-center gap-4 flex-1 min-w-0">
                <Subtitle class="text-primary">
                    {
                        encounter.period.as_ref().and_then(|p| p.start)
                        .map_or_else(
                            || "-".to_string(),
                            |s| s.format("%d").to_string())
                    }
                </Subtitle>
                <div class="flex flex-col min-w-0">
                    <NormalText class="font-semibold truncate">
                        {match &encounter.class {
                            EncounterClass::Ambulatory => "Consulta Ambulatoria",
                            EncounterClass::Emergency => "Emergencia",
                            EncounterClass::HomeHealth => "Domiciliar",
                            EncounterClass::Virtual => "Virtual",
                            EncounterClass::Field => "De Campo",
                            EncounterClass::Inpatient => "Hospitalización",
                            EncounterClass::Acute => "Atención Aguda",
                        }}
                    </NormalText>
                    <MutedText class="text-sm">
                        {date.format("%d/%m/%Y").to_string()}
                    </MutedText>
                </div>
            </div>
            <div>
                {match &encounter.status {
                    EncounterStatus::Planned => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-blue-100 text-blue-800 rounded-md text-xs font-medium">
                            <crate::components::Calendar class="size-3" />
                            {"Planificada"}
                        </span>
                    },
                    EncounterStatus::Finished => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-green-100 text-green-800 rounded-md text-xs font-medium">
                            <crate::components::Check class="size-3" />
                            {"Finalizada"}
                        </span>
                    },
                    EncounterStatus::Cancelled => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-red-100 text-red-800 rounded-md text-xs font-medium">
                            <crate::components::X class="size-3" />
                            {"Cancelada"}
                        </span>
                    },
                    EncounterStatus::Arrived => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-yellow-100 text-yellow-800 rounded-md text-xs font-medium">
                            {"Llegó"}
                        </span>
                    },
                    EncounterStatus::InProgress => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-purple-100 text-purple-800 rounded-md text-xs font-medium">                            {"En Progreso"}
                        </span>
                    },
                    EncounterStatus::Triaged => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-orange-100 text-orange-800 rounded-md text-xs font-medium">
                            {"Triaje"}
                        </span>
                    },
                    EncounterStatus::Onleave => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-gray-100 text-gray-800 rounded-md text-xs font-medium">
                            {"Ausente"}
                        </span>
                    },
                    EncounterStatus::EnteredInError => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-red-100 text-red-800 rounded-md text-xs font-medium">
                            {"Error"}
                        </span>
                    },
                    EncounterStatus::Unknown => html! {
                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-gray-100 text-gray-800 rounded-md text-xs font-medium">
                            {"Desconocido"}
                        </span>
                    },
                }}
            </div>
        </yew_router::components::Link::<crate::router::Route>>
    }
}

