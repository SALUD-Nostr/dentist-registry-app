//! Encounter detail view with clinical impressions

use chrono::Timelike;
use gloo_console::log;
use salud_types::{
    ClinicalImpression, ClinicalImpressionStatus, Encounter, EncounterClass, EncounterStatus,
    Patient,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typography::{Label, MutedText, NormalText, Subtitle, Title};
use crate::components::{Button, ButtonSize, ButtonVariant};

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
    let nostr_key = nostr_minions::use_nostr_key();

    let encounter = use_state(|| None::<Encounter>);
    let patient = use_state(|| None::<Patient>);
    let clinical_impressions = use_state(Vec::<ClinicalImpression>::new);
    let is_loading = use_state(|| true);
    let is_loading_impressions = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load encounter and patient on mount
    {
        let encounter = encounter.clone();
        let patient = patient.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let encounter_store = encounter_store.clone();

        use_effect_with(props.encounter_id.clone(), move |id| {
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
                        error.set(Some(format!("Error al cargar la cita: {e:?}",)));
                        is_loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Load clinical impressions for this encounter
    {
        let clinical_impressions = clinical_impressions.clone();
        let is_loading_impressions = is_loading_impressions.clone();

        use_effect_with(props.encounter_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                log!("Loading clinical impressions for encounter:", id.as_str());
                match clinical_impression_store.get_by_encounter(&id).await {
                    Ok(impressions) => {
                        log!(
                            "Clinical impressions loaded:",
                            format!("{} impressions", impressions.len())
                        );
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
        Callback::from(move |_: MouseEvent| {
            navigator.push(&crate::router::Route::History);
        })
    };

    let handle_new_impression = {
        let encounter_id = props.encounter_id.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&crate::router::Route::ClinicalImpressionNew {
                encounter_id: encounter_id.clone(),
            });
        })
    };

    let handle_mark_completed = {
        let encounter = encounter.clone();
        let encounter_store = encounter_store.clone();
        let encounter_id = props.encounter_id.clone();
        let nostr_key = nostr_key.clone();

        Callback::from(move |_: MouseEvent| {
            let encounter = encounter.clone();
            let encounter_store = encounter_store.clone();
            let encounter_id = encounter_id.clone();
            let nostr_key = nostr_key.clone();

            spawn_local(async move {
                if let Some(mut enc) = (*encounter).clone() {
                    enc.status = EncounterStatus::Finished;

                    // Get keypair
                    let keypair = match nostr_key.as_ref() {
                        Some(key) => key,
                        None => {
                            log!("Error: No keypair available");
                            return;
                        }
                    };

                    match encounter_store.save(&enc, keypair).await {
                        Ok(_) => {
                            log!("Encounter marked as completed");
                            // Reload the encounter
                            if let Ok(Some(updated_enc)) = encounter_store.get(&encounter_id).await
                            {
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
        let nostr_key = nostr_key.clone();

        Callback::from(move |_: MouseEvent| {
            let encounter = encounter.clone();
            let encounter_store = encounter_store.clone();
            let encounter_id = encounter_id.clone();
            let nostr_key = nostr_key.clone();

            spawn_local(async move {
                if let Some(mut enc) = (*encounter).clone() {
                    enc.status = EncounterStatus::Cancelled;

                    // Get keypair
                    let keypair = match nostr_key.as_ref() {
                        Some(key) => key,
                        None => {
                            log!("Error: No keypair available");
                            return;
                        }
                    };

                    match encounter_store.save(&enc, keypair).await {
                        Ok(()) => {
                            log!("Encounter marked as cancelled");
                            // Reload the encounter
                            if let Ok(Some(updated_enc)) = encounter_store.get(&encounter_id).await
                            {
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

    let Some(patient) = patient.as_ref() else {
        return html! {
            <div class="flex flex-col items-center gap-4 py-12">
                <crate::components::X class="size-16 text-red-600" />
                <p class="text-red-600 font-semibold text-lg">{"No se encontró el paciente"}</p>
                <Button
                    variant={ButtonVariant::Primary}
                    size={ButtonSize::Medium}
                    onclick={Some(handle_back.clone())}
                >
                    {"Volver al historial"}
                </Button>
            </div>
        };
    };

    let Some(encounter) = encounter.as_ref() else {
        return html! {
            <div class="flex flex-col items-center gap-4 py-12">
                <crate::components::X class="size-16 text-red-600" />
                <p class="text-red-600 font-semibold text-lg">{"No se encontró la cita"}</p>
                <Button
                    variant={ButtonVariant::Primary}
                    size={ButtonSize::Medium}
                    onclick={Some(handle_back.clone())}
                >
                    {"Volver al historial"}
                </Button>
            </div>
        };
    };

    html! {
        <div class="flex flex-col size-full detail-page overflow-auto">
            <div class="max-w-4xl mx-auto w-full">
                <div class="mb-4">
                    <Button
                        variant={ButtonVariant::Text}
                        size={ButtonSize::Medium}
                        onclick={Some(handle_back.clone())}
                        class="mb-3"
                    >
                        <crate::components::ArrowLeft class="size-4" />
                        {"Volver al Historial"}
                    </Button>
                    <Title>
                        {"Detalles de la Cita"}
                    </Title>
                    <MutedText class="text-xs">{"ID: "}{encounter.id.as_ref().unwrap_or(&props.encounter_id)}</MutedText>
                </div>

                if *is_loading {
                    <div class="flex items-center justify-center py-8">
                        <div class="flex flex-col items-center gap-4">
                            <div class="size-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
                            <MutedText>{"Cargando información de la cita..."}</MutedText>
                        </div>
                    </div>
                } else if let Some(err) = (*error).as_ref() {
                    <shady_minions::ui::Card>
                        <div class="flex flex-col items-center gap-4 py-12">
                            <crate::components::X class="size-16 text-red-600" />
                            <p class="text-red-600 font-semibold text-lg">{err}</p>
                            <Button
                                variant={ButtonVariant::Primary}
                                size={ButtonSize::Medium}
                                onclick={Some(handle_back.clone())}
                            >
                                {"Volver al historial"}
                            </Button>
                        </div>
                    </shady_minions::ui::Card>
                } else  {
                    <div class="grid gap-4">
                        // Encounter Information
                        <EncounterDetailCard
                            encounter={encounter.clone()}
                            patient={patient.clone()}
                            on_mark_cancelled={handle_mark_cancelled}
                            on_mark_completed={handle_mark_completed}
                        />



                        // Clinical Impressions Card
                        <shady_minions::ui::Card class="!border-0 !shadow-none">
                            <div class="flex justify-between items-center mb-3">
                                <Subtitle>{"Impresiones Clínicas"}</Subtitle>
                                {if encounter.status != EncounterStatus::Finished && encounter.status != EncounterStatus::Cancelled {
                                    html! {
                                        <Button
                                            variant={ButtonVariant::Primary}
                                            size={ButtonSize::Small}
                                            onclick={Some(handle_new_impression)}
                                        >
                                            <crate::components::Plus class="size-4" />
                                            {"Agregar"}
                                        </Button>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>

                            if *is_loading_impressions {
                                <div class="text-center py-8">
                                    <div class="size-8 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto"></div>
                                    <MutedText class="mt-4">{"Cargando impresiones clínicas..."}</MutedText>
                                </div>
                            } else if (*clinical_impressions).is_empty() {
                                <div class="text-center py-8">
                                    <crate::components::Clipboard class="size-16 mx-auto mb-4 opacity-50 text-muted" />
                                    <MutedText>{"No hay impresiones clínicas registradas"}</MutedText>
                                    {
                                    if encounter.status != EncounterStatus::Finished && encounter.status != EncounterStatus::Cancelled {
                                        html! {
                                            <MutedText class="mt-2">{"Haz clic en 'Agregar' para crear la primera impresión"}</MutedText>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                </div>
                            } else {
                                <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
                                    { for (*clinical_impressions).iter().map(|impression| {
                                        html! { <ClinicalImpressionCard impression={impression.clone()} /> }
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
struct EncounterStatusBadgeProps {
    pub status: EncounterStatus,
}

#[function_component(EncounterStatusBadge)]
fn encounter_status_badge(props: &EncounterStatusBadgeProps) -> Html {
    let class = match props.status {
        EncounterStatus::Planned => "bg-blue-100 text-blue-800",
        EncounterStatus::Arrived => "bg-yellow-100 text-yellow-800",
        EncounterStatus::Triaged => "bg-orange-100 text-orange-800",
        EncounterStatus::InProgress => "bg-purple-100 text-purple-800",
        EncounterStatus::Finished => "bg-green-100 text-green-800",
        EncounterStatus::Cancelled | EncounterStatus::EnteredInError => "bg-red-100 text-red-800",
        EncounterStatus::Onleave | EncounterStatus::Unknown => "bg-gray-100 text-gray-800",
    };

    let inner_text = match props.status {
        EncounterStatus::Planned => "Planificada",
        EncounterStatus::Arrived => "Llegó",
        EncounterStatus::Triaged => "Triaje",
        EncounterStatus::InProgress => "En Progreso",
        EncounterStatus::Onleave => "Ausente",
        EncounterStatus::Finished => "Finalizada",
        EncounterStatus::Cancelled => "Cancelada",
        EncounterStatus::EnteredInError => "Error",
        EncounterStatus::Unknown => "Desconocido",
    };

    html! {
        <span class={classes!(class, "inline-flex",  "items-center", "gap-1", "px-2", "py-1", "rounded-md", "text-xs", "font-medium")}>
            <crate::components::Calendar class="size-4" />
            {inner_text}
        </span>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct EncounterDetailCardProps {
    pub encounter: Encounter,
    pub patient: Patient,
    #[prop_or_default]
    pub on_mark_cancelled: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_mark_completed: Callback<MouseEvent>,
}

#[function_component(EncounterDetailCard)]
fn encounter_detail(props: &EncounterDetailCardProps) -> Html {
    let EncounterDetailCardProps {
        encounter,
        patient,
        on_mark_cancelled,
        on_mark_completed,
    } = props;
    let Some(period) = encounter.period.as_ref() else {
        return html! {};
    };
    let Some(start) = period.start else {
        return html! {};
    };
    // Stored as UTC; display in the clinic's local timezone.
    let start = start.with_timezone(&chrono::Local);
    html! {
        <shady_minions::ui::Card class="!border-0 !shadow-none">
            <div class="flex justify-between items-start mb-4 flex-col md:flex-row gap-2">
                <Subtitle class="flex items-center gap-2">
                    <crate::components::Stethoscope class="size-5 text-primary" />
                    {"Información de la Cita"}
                </Subtitle>
                {if encounter.status != EncounterStatus::Finished && encounter.status != EncounterStatus::Cancelled {
                    html! {
                        <div class="flex gap-2">
                            <Button
                                variant={ButtonVariant::Destructive}
                                size={ButtonSize::Small}
                                onclick={Some(props.on_mark_cancelled.clone())}
                            >
                                <crate::components::X class="size-4" />
                                {"Cancelar"}
                            </Button>
                            <Button
                                variant={ButtonVariant::Primary}
                                size={ButtonSize::Small}
                                onclick={Some(props.on_mark_completed.clone())}
                                class="!bg-gradient-to-b !from-green-600 !to-green-700 !shadow-green-600/30 hover:!shadow-green-600/40"
                            >
                                <crate::components::Check class="size-4" />
                                {"Completar"}
                            </Button>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
            <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
                // Patient
                <div>
                    <Label>{"Paciente"}</Label>
                    <NormalText class="font-medium">
                        {
                            patient.full_name().unwrap_or_else(|| "Paciente".to_string())
                        }
                    </NormalText>
                </div>

                // Status
                <div>
                    <Label>{"Estado"}</Label>
                    <EncounterStatusBadge status={encounter.status} />
                </div>

                // Class/Type
                <div>
                    <Label>
                        {"Tipo de Consulta"}
                    </Label>
                    <NormalText>
                        {
                            match &encounter.class {
                                EncounterClass::Ambulatory => "Ambulatoria",
                                EncounterClass::Emergency => "Emergencia",
                                EncounterClass::HomeHealth => "Domiciliar",
                                EncounterClass::Virtual => "Virtual",
                                EncounterClass::Field => "De Campo",
                                EncounterClass::Inpatient => "Hospitalización",
                                EncounterClass::Acute => "Atención Aguda",
                            }
                        }
                    </NormalText>
                </div>

                // Date and Time
                <div>
                    <Label>
                        {"Fecha"}
                    </Label>
                    <NormalText>
                        {start.format("%d/%m/%Y").to_string()}
                    </NormalText>
                </div>

                <div>
                    <Label>
                        {"Hora"}
                    </Label>
                    <NormalText>
                        {format!("{:02}:{:02}", start.hour(), start.minute())}
                    </NormalText>
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
                                format!("{hours} hora(s) {mins} min")
                            } else {
                                format!("{hours} hora(s)", )
                            }
                        } else {
                            format!("{minutes} minutos", )
                        };
                        html! {
                            <div>
                                <Label>
                                    {"Duración"}
                                </Label>
                                <NormalText>
                                    {duration_str}
                                </NormalText>
                            </div>
                        }
                    })
                }

                // Reason
                if let Some(reason_codes) = &encounter.reason_code {
                    if !reason_codes.is_empty() {
                        <div class="md:col-span-2">
                            <Label>
                                {"Motivo de Consulta"}
                            </Label>
                            <div class="space-y-2">
                                { for reason_codes.iter().map(|reason| {
                                    html! {
                                        <NormalText class="bg-gray-50 p-3 rounded-lg">
                                            {reason.text.clone()}
                                        </NormalText>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                }
            </div>
        </shady_minions::ui::Card>
    }
}

#[derive(Properties, PartialEq)]
struct ClinicalImpressionStatusBadgeProps {
    pub status: ClinicalImpressionStatus,
}

#[function_component(ClinicalImpressionStatusBadge)]
fn clinical_impression_status_badge(props: &ClinicalImpressionStatusBadgeProps) -> Html {
    let class = match props.status {
        ClinicalImpressionStatus::InProgress => "bg-purple-100 text-purple-800",
        ClinicalImpressionStatus::Completed => "bg-green-100 text-green-800",
        ClinicalImpressionStatus::EnteredInError => "bg-red-100 text-red-800",
    };

    let inner_text = match props.status {
        ClinicalImpressionStatus::InProgress => "En Progreso",
        ClinicalImpressionStatus::Completed => "Completada",
        ClinicalImpressionStatus::EnteredInError => "Error",
    };

    html! {
        <span class={classes!(class, "inline-flex",  "items-center", "gap-1", "px-2", "py-1", "rounded-md", "text-xs", "font-medium")}>
            <crate::components::Check class="size-4" />
            {inner_text}
        </span>
    }
}

#[derive(Properties, PartialEq)]
struct ClinicalImpressionProps {
    pub impression: ClinicalImpression,
}

#[function_component(ClinicalImpressionCard)]
fn clinical_impression_card(props: &ClinicalImpressionProps) -> Html {
    let ClinicalImpressionProps { impression } = props;
    let Some(date) = impression.date else {
        return html! {};
    };
    // Stored as UTC; display in the clinic's local timezone.
    let date = date.with_timezone(&chrono::Local);
    // Determine border color based on status
    let status_border_color = match impression.status {
        salud_types::ClinicalImpressionStatus::Completed => "border-l-green-500",
        salud_types::ClinicalImpressionStatus::InProgress => "border-l-blue-500",
        salud_types::ClinicalImpressionStatus::EnteredInError => "border-l-red-500",
    };

    html! {
        <div class={classes!(
            "relative",
            "border",
            "border-muted/20",
            "border-l-4",
            status_border_color,
            "rounded-lg",
            "p-3",
            "bg-white",
            "hover:shadow-lg",
            "transition-all",
            "duration-200",
            "shadow-sm"
        )}>
            // Compact header
            <div class="flex items-center justify-between gap-2 mb-2">
                <ClinicalImpressionStatusBadge status={impression.status} />
                <div class="flex items-center gap-1">
                    <crate::components::Calendar class="size-3 text-muted" />
                    <MutedText class="text-xs">
                        {date.format("%d/%m/%y %H:%M").to_string()}
                    </MutedText>
                </div>
            </div>

            // Summary - main content
            if let Some(summary) = &impression.summary {
                <div class="mb-2">
                    <NormalText class="text-sm text-foreground whitespace-pre-wrap">
                        {summary.clone()}
                    </NormalText>
                </div>
            }

            // Findings - compact list
            if let Some(findings) = &impression.finding && !findings.is_empty() {
                <ul class="mb-2 space-y-1">
                    { for findings.iter().map(|finding| {
                        finding.item_codeable_concept.as_ref().map_or_else(|| html! {}, |item| {
                            html! {
                                <li class="text-sm text-muted flex items-start gap-1.5">
                                    <span class="text-primary font-bold mt-0.5">{"•"}</span>
                                    <span class="flex-1">{&item.text}</span>
                                </li>
                            }
                        })
                    }) }
                </ul>
            }

            // Notes - inline if present
            if let Some(notes) = &impression.note && !notes.is_empty() {
                <div class="border-t border-muted/10 pt-2 mt-2 space-y-1.5">
                    { for notes.iter().map(|note| {
                        html! {
                            <div class="text-xs text-muted italic">
                                {note.text.clone()}
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
