//! Clinical impression form

use chrono::Utc;
use gloo_console::log;
use salud_types::{
    Annotation, ClinicalImpressionBuilder, ClinicalImpressionFinding, ClinicalImpressionStatus,
    CodeableConcept, Reference,
};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Button, ButtonVariant, ButtonSize};
use crate::components::typography::{Title, Subtitle, MutedText, Label};

#[derive(Properties, PartialEq, Eq)]
pub struct ClinicalImpressionFormProps {
    pub encounter_id: String,
}

#[function_component(ClinicalImpressionForm)]
pub fn clinical_impression_form(props: &ClinicalImpressionFormProps) -> Html {
    let navigator = use_navigator().unwrap();
    let clinical_impression_store = crate::storage::use_clinical_impression_store();
    let encounter_store = crate::storage::use_encounter_store();
    let nostr_key = nostr_minions::use_nostr_key();

    // Form state
    let summary = use_state(String::new);
    let findings = use_state(|| vec![String::new()]);
    let notes = use_state(String::new);
    let status = use_state(|| ClinicalImpressionStatus::InProgress);

    // Submission state
    let is_saving = use_state(|| false);
    let errors = use_state(|| Vec::<String>::new());

    // Patient subject reference (loaded from encounter)
    let patient_subject = use_state(|| None::<Reference>);

    // Load encounter to get patient reference
    {
        let encounter_id = props.encounter_id.clone();
        let patient_subject = patient_subject.clone();
        let encounter_store = encounter_store.clone();

        use_effect_with(encounter_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                if let Ok(Some(encounter)) = encounter_store.get(&id).await {
                    patient_subject.set(Some(encounter.subject));
                }
            });
            || ()
        });
    }

    let on_summary_change = {
        let summary = summary.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            summary.set(input.value());
        })
    };

    let on_finding_change = {
        let findings = findings.clone();
        Callback::from(move |(index, value): (usize, String)| {
            let mut current_findings = (*findings).clone();
            if index < current_findings.len() {
                current_findings[index] = value;
                findings.set(current_findings);
            }
        })
    };

    let on_add_finding = {
        let findings = findings.clone();
        Callback::from(move |_| {
            let mut current_findings = (*findings).clone();
            current_findings.push(String::new());
            findings.set(current_findings);
        })
    };

    let on_remove_finding = {
        let findings = findings.clone();
        Callback::from(move |index: usize| {
            let mut current_findings = (*findings).clone();
            if current_findings.len() > 1 && index < current_findings.len() {
                current_findings.remove(index);
                findings.set(current_findings);
            }
        })
    };

    let on_notes_change = {
        let notes = notes.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            notes.set(input.value());
        })
    };

    let on_status_change = {
        let status = status.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let new_status = match select.value().as_str() {
                "completed" => ClinicalImpressionStatus::Completed,
                "entered-in-error" => ClinicalImpressionStatus::EnteredInError,
                _ => ClinicalImpressionStatus::InProgress,
            };
            status.set(new_status);
        })
    };

    let on_submit = {
        let summary = summary.clone();
        let findings = findings.clone();
        let notes = notes.clone();
        let status = status.clone();
        let patient_subject = patient_subject.clone();
        let encounter_id = props.encounter_id.clone();
        let is_saving = is_saving.clone();
        let errors = errors.clone();
        let clinical_impression_store = clinical_impression_store.clone();
        let navigator = navigator.clone();
        let nostr_key = nostr_key.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Validation
            let mut validation_errors = Vec::new();
            if summary.trim().is_empty() {
                validation_errors.push("El resumen clínico es obligatorio".to_string());
            }

            if !validation_errors.is_empty() {
                errors.set(validation_errors);
                return;
            }

            errors.set(Vec::new());
            is_saving.set(true);

            let summary = (*summary).clone();
            let findings_vec = (*findings).clone();
            let notes_text = (*notes).clone();
            let status_val = (*status).clone();
            let patient_subject_val = (*patient_subject).clone();
            let encounter_id = encounter_id.clone();
            let is_saving = is_saving.clone();
            let errors = errors.clone();
            let clinical_impression_store = clinical_impression_store.clone();
            let navigator = navigator.clone();
            let nostr_key = nostr_key.clone();

            spawn_local(async move {
                // Build findings
                let mut findings_list = Vec::new();
                for finding_text in findings_vec {
                    if !finding_text.trim().is_empty() {
                        findings_list.push(ClinicalImpressionFinding {
                            item_codeable_concept: Some(CodeableConcept {
                                text: finding_text.trim().to_string(),
                                coding: None,
                            }),
                            basis: None,
                        });
                    }
                }

                // Build notes
                let mut notes_list = Vec::new();
                if !notes_text.trim().is_empty() {
                    notes_list.push(Annotation {
                        text: notes_text.trim().to_string(),
                        time: Some(Utc::now()),
                    });
                }

                // Build references
                let encounter_ref = Reference {
                    reference: Some(format!("Encounter/{}", encounter_id)),
                    type_: Some("Encounter".to_string()),
                    display: None,
                };

                let subject_ref = patient_subject_val.unwrap_or(Reference {
                    reference: None,
                    type_: Some("Patient".to_string()),
                    display: None,
                });

                // Build ClinicalImpression
                let mut builder = ClinicalImpressionBuilder::default();
                builder
                    .id(Uuid::new_v4().to_string())
                    .resource_type("ClinicalImpression".to_string())
                    .status(status_val)
                    .subject(subject_ref)
                    .encounter(encounter_ref)
                    .effective_date_time(Utc::now())
                    .date(Utc::now())
                    .summary(summary);

                if !findings_list.is_empty() {
                    builder.finding(findings_list);
                }

                if !notes_list.is_empty() {
                    builder.note(notes_list);
                }

                match builder.build() {
                    Ok(impression) => {
                        // Get keypair
                        let keypair = match nostr_key.as_ref() {
                            Some(key) => key,
                            None => {
                                log!("Error: No keypair available");
                                errors.set(vec!["No se encontró la clave de firma".to_string()]);
                                is_saving.set(false);
                                return;
                            }
                        };

                        match clinical_impression_store.save(&impression, keypair).await {
                            Ok(_) => {
                                log!("Clinical impression saved successfully");
                                navigator
                                    .push(&crate::router::Route::EncounterDetail { id: encounter_id });
                            }
                            Err(e) => {
                                log!("Error saving clinical impression:", format!("{:?}", e));
                                errors.set(vec![format!("Error al guardar: {:?}", e)]);
                                is_saving.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        log!("Error building clinical impression:", format!("{:?}", e));
                        errors.set(vec![format!("Error al crear impresión: {:?}", e)]);
                        is_saving.set(false);
                    }
                }
            });
        })
    };

    let handle_cancel = {
        let navigator = navigator;
        let encounter_id = props.encounter_id.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::EncounterDetail {
                id: encounter_id.clone(),
            });
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
                        {"Volver a la Cita"}
                    </button>
                    <Title>{"Nueva Impresión Clínica"}</Title>
                    <MutedText>{"Cita ID: "}{&props.encounter_id}</MutedText>
                </div>

                if !(*errors).is_empty() {
                    <div class="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
                        <div class="flex items-start gap-2">
                            <crate::components::X class="size-5 text-red-600 mt-0.5" />
                            <div class="flex-1">
                                <Subtitle size="text-sm font-semibold text-red-800">{"Errores de validación"}</Subtitle>
                                <ul class="mt-2 space-y-1">
                                    { for (*errors).iter().map(|error| html! {
                                        <li class="text-sm text-red-700">{error}</li>
                                    }) }
                                </ul>
                            </div>
                        </div>
                    </div>
                }

                <form onsubmit={on_submit}>
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="space-y-6">
                            // Summary
                            <div>
                                <Label class="text-foreground mb-2">
                                    {"Resumen Clínico"}
                                    <span class="text-red-600">{"*"}</span>
                                </Label>
                                <textarea
                                    value={(*summary).clone()}
                                    oninput={on_summary_change}
                                    rows="6"
                                    placeholder="Describa el resumen de la evaluación clínica..."
                                    class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary"
                                    required={true}
                                />
                                <MutedText size="mt-1 text-xs text-muted">
                                    {"Resumen general de la evaluación clínica del paciente"}
                                </MutedText>
                            </div>

                            // Findings
                            <div>
                                <div class="flex justify-between items-center mb-2">
                                    <Label class="text-foreground">
                                        {"Hallazgos y Diagnósticos"}
                                    </Label>
                                    <Button
                                        variant={ButtonVariant::Text}
                                        size={ButtonSize::Small}
                                        onclick={Some(on_add_finding)}
                                    >
                                        <crate::components::Plus class="size-4" />
                                        {"Agregar hallazgo"}
                                    </Button>
                                </div>
                                <div class="space-y-3">
                                    { for (*findings).iter().enumerate().map(|(index, finding)| {
                                        let on_change = {
                                            let on_finding_change = on_finding_change.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                on_finding_change.emit((index, input.value()));
                                            })
                                        };

                                        let on_remove = {
                                            let on_remove_finding = on_remove_finding.clone();
                                            Callback::from(move |_| {
                                                on_remove_finding.emit(index);
                                            })
                                        };

                                        html! {
                                            <div class="flex gap-2">
                                                <input
                                                    type="text"
                                                    value={finding.clone()}
                                                    oninput={on_change}
                                                    placeholder={format!("Hallazgo #{}", index + 1)}
                                                    class="flex-1 px-3 py-2 border border-muted rounded-lg focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary"
                                                />
                                                if (*findings).len() > 1 {
                                                    <Button
                                                        variant={ButtonVariant::Destructive}
                                                        size={ButtonSize::Small}
                                                        onclick={Some(on_remove)}
                                                        class="!px-2 !py-2"
                                                    >
                                                        <crate::components::X class="size-5" />
                                                    </Button>
                                                }
                                            </div>
                                        }
                                    }) }
                                </div>
                                <MutedText size="mt-1 text-xs text-muted">
                                    {"Hallazgos específicos, diagnósticos o condiciones identificadas"}
                                </MutedText>
                            </div>

                            // Notes
                            <div>
                                <Label class="text-foreground mb-2">
                                    {"Notas Adicionales"}
                                </Label>
                                <textarea
                                    value={(*notes).clone()}
                                    oninput={on_notes_change}
                                    rows="4"
                                    placeholder="Comentarios, observaciones o información adicional..."
                                    class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary"
                                />
                                <MutedText size="mt-1 text-xs text-muted">
                                    {"Comentarios adicionales sobre la evaluación"}
                                </MutedText>
                            </div>

                            // Status
                            <div>
                                <Label class="text-foreground mb-2">
                                    {"Estado de la Impresión"}
                                </Label>
                                <select
                                    onchange={on_status_change}
                                    class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary"
                                >
                                    <option value="in-progress" selected={*status == ClinicalImpressionStatus::InProgress}>
                                        {"En Progreso"}
                                    </option>
                                    <option value="completed" selected={*status == ClinicalImpressionStatus::Completed}>
                                        {"Completada"}
                                    </option>
                                    <option value="entered-in-error" selected={*status == ClinicalImpressionStatus::EnteredInError}>
                                        {"Ingresada con Error"}
                                    </option>
                                </select>
                                <MutedText size="mt-1 text-xs text-muted">
                                    {"Estado actual de esta evaluación clínica"}
                                </MutedText>
                            </div>

                            // Action buttons
                            <div class="flex justify-end gap-3 pt-4 border-t border-muted">
                                <Button
                                    variant={ButtonVariant::Outline}
                                    size={ButtonSize::Medium}
                                    onclick={Some(handle_cancel)}
                                    disabled={*is_saving}
                                >
                                    {"Cancelar"}
                                </Button>
                                <Button
                                    variant={ButtonVariant::Primary}
                                    size={ButtonSize::Medium}
                                    button_type="submit".to_string()
                                    loading={*is_saving}
                                    disabled={*is_saving}
                                >
                                    if !*is_saving {
                                        <crate::components::Check class="size-5" />
                                    }
                                    {if *is_saving { "Guardando..." } else { "Guardar Impresión" }}
                                </Button>
                            </div>
                        </div>
                    </shady_minions::ui::Card>
                </form>
            </div>
        </div>
    }
}
