//! Encounter detail view with clinical impressions

use chrono::Timelike;
use gloo_console::log;
use salud_types::{ClinicalImpression, ClinicalImpressionStatus, Encounter, EncounterClass, EncounterStatus, Patient};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EncounterDetailProps {
    pub encounter_id: String,
}

#[function_component(EncounterDetail)]
pub fn encounter_detail(props: &EncounterDetailProps) -> Html {
    let navigator = use_navigator().unwrap();
    let encounter_store = crate::storage::use_encounter_store();
    let patient_store = crate::storage::use_patient_store();
    let clinical_impression_store = crate::storage::use_clinical_impression_store();

    let encounter = use_state(|| None::<Encounter>);
    let patient = use_state(|| None::<Patient>);
    let clinical_impressions = use_state(|| Vec::<ClinicalImpression>::new());
    let is_loading = use_state(|| true);
    let is_loading_impressions = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load encounter and patient on mount
    {
        let encounter_id = props.encounter_id.clone();
        let encounter = encounter.clone();
        let patient = patient.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let encounter_store = encounter_store.clone();
        let patient_store = patient_store.clone();

        use_effect_with(encounter_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                log!("Loading encounter:", id.as_str());
                match encounter_store.get(&id).await {
                    Ok(Some(enc)) => {
                        log!("Encounter loaded successfully");

                        // Load patient if subject is present
                        if let Some(ref_str) = &enc.subject.reference {
                            let patient_id = ref_str.strip_prefix("Patient/").unwrap_or(ref_str);

                            match patient_store.get(patient_id).await {
                                Ok(Some(pat)) => {
                                    log!("Patient loaded successfully");
                                    patient.set(Some(pat));
                                }
                                Ok(None) => {
                                    log!("Patient not found");
                                }
                                Err(e) => {
                                    log!("Error loading patient:", format!("{:?}", e));
                                }
                            }
                        }

                        encounter.set(Some(enc));
                        is_loading.set(false);
                    }
                    Ok(None) => {
                        log!("Encounter not found");
                        error.set(Some("Cita no encontrada".to_string()));
                        is_loading.set(false);
                    }
                    Err(e) => {
                        log!("Error loading encounter:", format!("{:?}", e));
                        error.set(Some(format!("Error al cargar la cita: {:?}", e)));
                        is_loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Load clinical impressions for this encounter
    {
        let encounter_id = props.encounter_id.clone();
        let clinical_impressions = clinical_impressions.clone();
        let is_loading_impressions = is_loading_impressions.clone();
        let clinical_impression_store = clinical_impression_store.clone();

        use_effect_with(encounter_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                log!("Loading clinical impressions for encounter:", id.as_str());
                match clinical_impression_store.get_by_encounter(&id).await {
                    Ok(impressions) => {
                        log!("Clinical impressions loaded:", format!("{} impressions", impressions.len()));
                        clinical_impressions.set(impressions);
                        is_loading_impressions.set(false);
                    }
                    Err(e) => {
                        log!("Error loading clinical impressions:", format!("{:?}", e));
                        is_loading_impressions.set(false);
                    }
                }
            });
            || ()
        });
    }

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
                encounter_id: encounter_id.clone(),
            });
        })
    };

    let handle_mark_completed = {
        let encounter = encounter.clone();
        let encounter_store = encounter_store.clone();
        let encounter_id = props.encounter_id.clone();

        Callback::from(move |_| {
            let encounter = encounter.clone();
            let encounter_store = encounter_store.clone();
            let encounter_id = encounter_id.clone();

            spawn_local(async move {
                if let Some(mut enc) = (*encounter).as_ref().cloned() {
                    enc.status = EncounterStatus::Finished;

                    match encounter_store.save(&enc).await {
                        Ok(_) => {
                            log!("Encounter marked as completed");
                            // Reload the encounter
                            if let Ok(Some(updated_enc)) = encounter_store.get(&encounter_id).await {
                                encounter.set(Some(updated_enc));
                            }
                        }
                        Err(e) => {
                            log!("Error updating encounter:", format!("{:?}", e));
                        }
                    }
                }
            });
        })
    };

    let handle_mark_cancelled = {
        let encounter = encounter.clone();
        let encounter_store = encounter_store.clone();
        let encounter_id = props.encounter_id.clone();

        Callback::from(move |_| {
            let encounter = encounter.clone();
            let encounter_store = encounter_store.clone();
            let encounter_id = encounter_id.clone();

            spawn_local(async move {
                if let Some(mut enc) = (*encounter).as_ref().cloned() {
                    enc.status = EncounterStatus::Cancelled;

                    match encounter_store.save(&enc).await {
                        Ok(_) => {
                            log!("Encounter marked as cancelled");
                            // Reload the encounter
                            if let Ok(Some(updated_enc)) = encounter_store.get(&encounter_id).await {
                                encounter.set(Some(updated_enc));
                            }
                        }
                        Err(e) => {
                            log!("Error updating encounter:", format!("{:?}", e));
                        }
                    }
                }
            });
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
                        {"Volver al Historial"}
                    </button>

                    if let Some(enc) = (*encounter).as_ref() {
                        <h1 class="text-3xl font-bold">
                            {"Detalles de la Cita"}
                            {
                                if let Some(pat) = (*patient).as_ref() {
                                    format!(" - {}", pat.full_name().unwrap_or_else(|| "Paciente".to_string()))
                                } else {
                                    String::new()
                                }
                            }
                        </h1>
                        <p class="text-sm text-muted">{"ID: "}{enc.id.as_ref().unwrap_or(&props.encounter_id)}</p>
                    } else {
                        <h1 class="text-3xl font-bold">{"Detalles de la Cita"}</h1>
                        <p class="text-sm text-muted">{"ID: "}{&props.encounter_id}</p>
                    }
                </div>

                if *is_loading {
                    <div class="flex items-center justify-center py-12">
                        <div class="flex flex-col items-center gap-4">
                            <div class="size-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
                            <p class="text-muted">{"Cargando información de la cita..."}</p>
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
                                {"Volver al historial"}
                            </button>
                        </div>
                    </shady_minions::ui::Card>
                } else if let Some(enc) = (*encounter).as_ref() {
                    <div class="grid gap-6">
                        // Encounter Information Card
                        <shady_minions::ui::Card>
                            <h2 class="text-xl font-semibold mb-6 flex items-center gap-2">
                                <crate::components::Stethoscope class="size-6 text-primary" />
                                {"Información de la Cita"}
                            </h2>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                // Patient
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Paciente"}
                                    </label>
                                    <p class="text-base font-medium text-foreground">
                                        {
                                            if let Some(pat) = (*patient).as_ref() {
                                                pat.full_name().unwrap_or_else(|| "Paciente".to_string())
                                            } else {
                                                enc.subject.reference.as_ref()
                                                    .map(|r| r.strip_prefix("Patient/").unwrap_or(r).to_string())
                                                    .unwrap_or_else(|| "Paciente".to_string())
                                            }
                                        }
                                    </p>
                                </div>

                                // Status
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Estado"}
                                    </label>
                                    <p class="text-base">
                                        {
                                            match &enc.status {
                                                EncounterStatus::Planned => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-blue-100 text-blue-800 rounded-md text-sm font-medium">
                                                        <crate::components::Calendar class="size-4" />
                                                        {"Planificada"}
                                                    </span>
                                                },
                                                EncounterStatus::Arrived => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-yellow-100 text-yellow-800 rounded-md text-sm font-medium">
                                                        {"Llegó"}
                                                    </span>
                                                },
                                                EncounterStatus::Triaged => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-orange-100 text-orange-800 rounded-md text-sm font-medium">
                                                        {"Triaje"}
                                                    </span>
                                                },
                                                EncounterStatus::InProgress => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-purple-100 text-purple-800 rounded-md text-sm font-medium">
                                                        {"En Progreso"}
                                                    </span>
                                                },
                                                EncounterStatus::Onleave => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-gray-100 text-gray-800 rounded-md text-sm font-medium">
                                                        {"Ausente"}
                                                    </span>
                                                },
                                                EncounterStatus::Finished => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-green-100 text-green-800 rounded-md text-sm font-medium">
                                                        <crate::components::Check class="size-4" />
                                                        {"Finalizada"}
                                                    </span>
                                                },
                                                EncounterStatus::Cancelled => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-red-100 text-red-800 rounded-md text-sm font-medium">
                                                        <crate::components::X class="size-4" />
                                                        {"Cancelada"}
                                                    </span>
                                                },
                                                EncounterStatus::EnteredInError => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-red-100 text-red-800 rounded-md text-sm font-medium">
                                                        {"Error"}
                                                    </span>
                                                },
                                                EncounterStatus::Unknown => html! {
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 bg-gray-100 text-gray-800 rounded-md text-sm font-medium">
                                                        {"Desconocido"}
                                                    </span>
                                                },
                                            }
                                        }
                                    </p>
                                </div>

                                // Class/Type
                                <div>
                                    <label class="block text-sm font-medium text-muted mb-1">
                                        {"Tipo de Consulta"}
                                    </label>
                                    <p class="text-base text-foreground">
                                        {
                                            match &enc.class {
                                                EncounterClass::Ambulatory => "Consulta Ambulatoria",
                                                EncounterClass::Emergency => "Emergencia",
                                                EncounterClass::HomeHealth => "Consulta a Domicilio",
                                                EncounterClass::Virtual => "Consulta Virtual",
                                                EncounterClass::Field => "Consulta en Terreno",
                                                EncounterClass::Inpatient => "Hospitalización",
                                                EncounterClass::Acute => "Atención Aguda",
                                            }
                                        }
                                    </p>
                                </div>

                                // Date and Time
                                if let Some(period) = &enc.period {
                                    if let Some(start) = period.start {
                                        <div>
                                            <label class="block text-sm font-medium text-muted mb-1">
                                                {"Fecha"}
                                            </label>
                                            <p class="text-base text-foreground">
                                                {start.format("%d/%m/%Y").to_string()}
                                            </p>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-muted mb-1">
                                                {"Hora"}
                                            </label>
                                            <p class="text-base text-foreground">
                                                {format!("{:02}:{:02}", start.hour(), start.minute())}
                                            </p>
                                        </div>

                                        // Duration
                                        {
                                            period.end.map(|end| {
                                                let duration = end.signed_duration_since(start);
                                                let minutes = duration.num_minutes();
                                                let duration_str = if minutes >= 60 {
                                                    let hours = minutes / 60;
                                                    let mins = minutes % 60;
                                                    if mins > 0 {
                                                        format!("{} hora(s) {} min", hours, mins)
                                                    } else {
                                                        format!("{} hora(s)", hours)
                                                    }
                                                } else {
                                                    format!("{} minutos", minutes)
                                                };
                                                html! {
                                                    <div>
                                                        <label class="block text-sm font-medium text-muted mb-1">
                                                            {"Duración"}
                                                        </label>
                                                        <p class="text-base text-foreground">
                                                            {duration_str}
                                                        </p>
                                                    </div>
                                                }
                                            })
                                        }
                                    }
                                }

                                // Reason
                                if let Some(reason_codes) = &enc.reason_code {
                                    if !reason_codes.is_empty() {
                                        <div class="md:col-span-2">
                                            <label class="block text-sm font-medium text-muted mb-1">
                                                {"Motivo de Consulta"}
                                            </label>
                                            <div class="space-y-2">
                                                { for reason_codes.iter().map(|reason| {
                                                    html! {
                                                        <p class="text-base text-foreground bg-gray-50 p-3 rounded-lg">
                                                            {&reason.text}
                                                        </p>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                    }
                                }
                            </div>
                        </shady_minions::ui::Card>

                        // Action Buttons
                        {
                            if enc.status != EncounterStatus::Finished && enc.status != EncounterStatus::Cancelled {
                                html! {
                                    <div class="flex gap-3 justify-end">
                                        <button
                                            onclick={handle_mark_cancelled.clone()}
                                            class="px-4 py-2 border border-red-600 text-red-600 rounded-lg hover:bg-red-50 transition-colors flex items-center gap-2"
                                        >
                                            <crate::components::X class="size-5" />
                                            {"Marcar como Cancelada"}
                                        </button>
                                        <button
                                            onclick={handle_mark_completed.clone()}
                                            class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors flex items-center gap-2"
                                        >
                                            <crate::components::Check class="size-5" />
                                            {"Marcar como Completada"}
                                        </button>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }

                        // Clinical Impressions Card
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

                            if *is_loading_impressions {
                                <div class="text-center py-8">
                                    <div class="size-8 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto"></div>
                                    <p class="text-muted mt-4">{"Cargando impresiones clínicas..."}</p>
                                </div>
                            } else if (*clinical_impressions).is_empty() {
                                <div class="text-center py-8">
                                    <crate::components::Clipboard class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                                    <p class="text-muted">{"No hay impresiones clínicas registradas"}</p>
                                    <p class="text-sm text-muted mt-2">{"Haz clic en 'Agregar' para crear la primera impresión"}</p>
                                </div>
                            } else {
                                <div class="space-y-4">
                                    { for (*clinical_impressions).iter().map(|impression| {
                                        html! {
                                            <div class="border border-muted rounded-lg p-4 hover:bg-gray-50 transition-colors">
                                                <div class="flex justify-between items-start mb-3">
                                                    <div class="flex-1">
                                                        <div class="flex items-center gap-2 mb-2">
                                                            {
                                                                match &impression.status {
                                                                    ClinicalImpressionStatus::InProgress => html! {
                                                                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-purple-100 text-purple-800 rounded-md text-xs font-medium">
                                                                            {"En Progreso"}
                                                                        </span>
                                                                    },
                                                                    ClinicalImpressionStatus::Completed => html! {
                                                                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-green-100 text-green-800 rounded-md text-xs font-medium">
                                                                            <crate::components::Check class="size-3" />
                                                                            {"Completada"}
                                                                        </span>
                                                                    },
                                                                    ClinicalImpressionStatus::EnteredInError => html! {
                                                                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-red-100 text-red-800 rounded-md text-xs font-medium">
                                                                            <crate::components::X class="size-3" />
                                                                            {"Error"}
                                                                        </span>
                                                                    },
                                                                }
                                                            }
                                                            if let Some(date) = impression.date {
                                                                <span class="text-xs text-muted">
                                                                    {date.format("%d/%m/%Y %H:%M").to_string()}
                                                                </span>
                                                            }
                                                        </div>
                                                    </div>
                                                </div>

                                                // Summary
                                                if let Some(summary) = &impression.summary {
                                                    <div class="mb-3">
                                                        <h4 class="text-sm font-semibold text-foreground mb-1">{"Resumen"}</h4>
                                                        <p class="text-sm text-foreground whitespace-pre-wrap">{summary}</p>
                                                    </div>
                                                }

                                                // Findings
                                                if let Some(findings) = &impression.finding {
                                                    if !findings.is_empty() {
                                                        <div class="mb-3">
                                                            <h4 class="text-sm font-semibold text-foreground mb-2">{"Hallazgos y Diagnósticos"}</h4>
                                                            <ul class="space-y-1">
                                                                { for findings.iter().map(|finding| {
                                                                    if let Some(item) = &finding.item_codeable_concept {
                                                                        html! {
                                                                            <li class="text-sm text-foreground flex items-start gap-2">
                                                                                <span class="text-primary mt-1">{"•"}</span>
                                                                                <span>{&item.text}</span>
                                                                            </li>
                                                                        }
                                                                    } else {
                                                                        html! {}
                                                                    }
                                                                }) }
                                                            </ul>
                                                        </div>
                                                    }
                                                }

                                                // Notes
                                                if let Some(notes) = &impression.note {
                                                    if !notes.is_empty() {
                                                        <div>
                                                            <h4 class="text-sm font-semibold text-foreground mb-2">{"Notas"}</h4>
                                                            { for notes.iter().map(|note| {
                                                                html! {
                                                                    <div class="text-sm text-muted bg-gray-50 p-2 rounded">
                                                                        <p class="whitespace-pre-wrap">{&note.text}</p>
                                                                        if let Some(time) = note.time {
                                                                            <p class="text-xs text-muted mt-1">
                                                                                {time.format("%d/%m/%Y %H:%M").to_string()}
                                                                            </p>
                                                                        }
                                                                    </div>
                                                                }
                                                            }) }
                                                        </div>
                                                    }
                                                }
                                            </div>
                                        }
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
