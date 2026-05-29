//! Medical intake form - the 5 key initial questions.
//!
//! Captures: age (from the patient record), reason for consult, last dental
//! visit, allergies, and systemic conditions. Allergies are stored as
//! `AllergyIntolerance`, systemic conditions as `Condition`, and the last
//! dental visit as a `Procedure`, all referencing the patient. The reason for
//! consult is kept on the intake's free-text notes.
//!
//! The form is intentionally non-linear: every section is visible and editable
//! at once, and re-opening the intake pre-loads what was previously saved.

use chrono::{Datelike, NaiveDate, Utc};
use gloo_console::log;
use salud_types::{
    AllergyIntoleranceBuilder, CodeableConcept, ConditionBuilder, Patient, ProcedureBuilder,
    ProcedureStatus, Reference,
};
use std::collections::HashSet;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typography::{Label, MutedText, Subtitle, Title};
use crate::components::{Button, ButtonSize, ButtonVariant};

/// Common allergy options offered as checkboxes (El Salvador dental context).
const COMMON_ALLERGIES: &[&str] = &[
    "Penicilina",
    "Anestesia (lidocaína)",
    "AINEs (ibuprofeno, etc.)",
    "Sulfas",
    "Latex",
    "Aspirina",
];

/// Common systemic conditions offered as checkboxes.
const COMMON_CONDITIONS: &[&str] = &[
    "Diabetes",
    "Hipertensión",
    "Cardiopatía",
    "Asma",
    "Embarazo",
    "Trastorno de coagulación",
];

#[derive(Properties, PartialEq, Eq)]
pub struct MedicalIntakeFormProps {
    pub patient_id: String,
}

#[function_component(MedicalIntakeForm)]
pub fn medical_intake_form(props: &MedicalIntakeFormProps) -> Html {
    let navigator = use_navigator().unwrap();
    let patient_store = crate::storage::use_patient_store();
    let allergy_store = crate::storage::use_allergy_store();
    let condition_store = crate::storage::use_condition_store();
    let procedure_store = crate::storage::use_procedure_store();
    let nostr_key = nostr_minions::use_nostr_key();

    let patient = use_state(|| None::<Patient>);

    // Section state.
    let reason = use_state(String::new);
    let last_visit_date = use_state(String::new);
    let last_visit_note = use_state(String::new);
    // Checked common items, plus a free-text "other" field for each.
    let allergies = use_state(HashSet::<String>::new);
    let allergy_other = use_state(String::new);
    let conditions = use_state(HashSet::<String>::new);
    let condition_other = use_state(String::new);

    let is_saving = use_state(|| false);
    let error = use_state(|| None::<String>);

    // Load patient + any previously-saved intake data on mount.
    {
        let patient = patient.clone();
        let allergies = allergies.clone();
        let allergy_other = allergy_other.clone();
        let conditions = conditions.clone();
        let condition_other = condition_other.clone();
        let last_visit_date = last_visit_date.clone();
        let last_visit_note = last_visit_note.clone();
        let patient_store = patient_store.clone();
        let allergy_store = allergy_store.clone();
        let condition_store = condition_store.clone();
        let procedure_store = procedure_store.clone();

        use_effect_with(props.patient_id.clone(), move |id| {
            let id = id.clone();
            spawn_local(async move {
                if let Ok(Some(p)) = patient_store.get(&id).await {
                    patient.set(Some(p));
                }

                let common_allergies: HashSet<&str> = COMMON_ALLERGIES.iter().copied().collect();
                if let Ok(existing) = allergy_store.get_by_patient(&id).await {
                    let mut checked = HashSet::new();
                    let mut others = Vec::new();
                    for a in existing {
                        if let Some(text) = a.code.map(|c| c.text) {
                            if common_allergies.contains(text.as_str()) {
                                checked.insert(text);
                            } else {
                                others.push(text);
                            }
                        }
                    }
                    allergies.set(checked);
                    if !others.is_empty() {
                        allergy_other.set(others.join(", "));
                    }
                }

                let common_conditions: HashSet<&str> = COMMON_CONDITIONS.iter().copied().collect();
                if let Ok(existing) = condition_store.get_by_patient(&id).await {
                    let mut checked = HashSet::new();
                    let mut others = Vec::new();
                    for c in existing {
                        if let Some(text) = c.code.map(|cc| cc.text) {
                            if common_conditions.contains(text.as_str()) {
                                checked.insert(text);
                            } else {
                                others.push(text);
                            }
                        }
                    }
                    conditions.set(checked);
                    if !others.is_empty() {
                        condition_other.set(others.join(", "));
                    }
                }

                if let Ok(procs) = procedure_store.get_by_patient(&id).await {
                    // Most recent dental-visit procedure pre-fills the section.
                    if let Some(p) = procs.into_iter().next() {
                        if let Some(dt) = p.performed_date_time {
                            last_visit_date.set(dt.format("%Y-%m-%d").to_string());
                        }
                        if let Some(notes) = p.note {
                            if let Some(n) = notes.into_iter().next() {
                                last_visit_note.set(n.text);
                            }
                        }
                    }
                }
            });
            || ()
        });
    }

    let toggle_set = |state: UseStateHandle<HashSet<String>>, item: String| {
        Callback::from(move |_: MouseEvent| {
            let mut set = (*state).clone();
            if !set.remove(&item) {
                set.insert(item.clone());
            }
            state.set(set);
        })
    };

    let patient_age = patient.as_ref().and_then(|p| {
        p.birth_date.map(|bd| {
            let today = Utc::now().date_naive();
            let mut age = today.year() - bd.year();
            if (today.month(), today.day()) < (bd.month(), bd.day()) {
                age -= 1;
            }
            age
        })
    });

    let handle_save = {
        let navigator = navigator.clone();
        let patient_id = props.patient_id.clone();
        let allergy_store = allergy_store.clone();
        let condition_store = condition_store.clone();
        let procedure_store = procedure_store.clone();
        let allergies = allergies.clone();
        let allergy_other = allergy_other.clone();
        let conditions = conditions.clone();
        let condition_other = condition_other.clone();
        let last_visit_date = last_visit_date.clone();
        let last_visit_note = last_visit_note.clone();
        let is_saving = is_saving.clone();
        let error = error.clone();
        let nostr_key = nostr_key.clone();

        Callback::from(move |_: MouseEvent| {
            let Some(keypair) = nostr_key.as_ref().cloned() else {
                error.set(Some("No se encontró la clave de firma".to_string()));
                return;
            };

            is_saving.set(true);
            error.set(None);

            let navigator = navigator.clone();
            let patient_id = patient_id.clone();
            let patient_ref = format!("Patient/{patient_id}");
            let allergy_store = allergy_store.clone();
            let condition_store = condition_store.clone();
            let procedure_store = procedure_store.clone();
            let is_saving = is_saving.clone();

            // Collect the final lists from form state.
            let mut allergy_list: Vec<String> = (*allergies).iter().cloned().collect();
            for extra in allergy_other.split(',') {
                let t = extra.trim();
                if !t.is_empty() {
                    allergy_list.push(t.to_string());
                }
            }
            let mut condition_list: Vec<String> = (*conditions).iter().cloned().collect();
            for extra in condition_other.split(',') {
                let t = extra.trim();
                if !t.is_empty() {
                    condition_list.push(t.to_string());
                }
            }
            let visit_date = (*last_visit_date).clone();
            let visit_note = (*last_visit_note).clone();

            spawn_local(async move {
                // Re-create from scratch: clear any prior intake-sourced records
                // for this patient, then write the current state. Simple upsert.
                if let Ok(old) = allergy_store.get_by_patient(&patient_id).await {
                    for a in old {
                        if let Some(id) = a.id {
                            let _ = allergy_store.delete(&id).await;
                        }
                    }
                }
                if let Ok(old) = condition_store.get_by_patient(&patient_id).await {
                    for c in old {
                        if let Some(id) = c.id {
                            let _ = condition_store.delete(&id).await;
                        }
                    }
                }
                if let Ok(old) = procedure_store.get_by_patient(&patient_id).await {
                    for p in old {
                        if let Some(id) = p.id {
                            let _ = procedure_store.delete(&id).await;
                        }
                    }
                }

                let subject = || Reference {
                    reference: Some(patient_ref.clone()),
                    type_: Some("Patient".to_string()),
                    display: None,
                };

                for text in allergy_list {
                    let allergy = AllergyIntoleranceBuilder::default()
                        .id(Uuid::new_v4().to_string())
                        .code(CodeableConcept { text, coding: None })
                        .patient(subject())
                        .recorded_date(Utc::now())
                        .build();
                    if let Ok(allergy) = allergy {
                        if let Err(e) = allergy_store.save(&allergy, &keypair).await {
                            log!("Error saving allergy:", format!("{:?}", e));
                        }
                    }
                }

                for text in condition_list {
                    let condition = ConditionBuilder::default()
                        .id(Uuid::new_v4().to_string())
                        .code(CodeableConcept { text, coding: None })
                        .subject(subject())
                        .recorded_date(Utc::now())
                        .build();
                    if let Ok(condition) = condition {
                        if let Err(e) = condition_store.save(&condition, &keypair).await {
                            log!("Error saving condition:", format!("{:?}", e));
                        }
                    }
                }

                if !visit_date.trim().is_empty() || !visit_note.trim().is_empty() {
                    let performed = NaiveDate::parse_from_str(&visit_date, "%Y-%m-%d")
                        .ok()
                        .and_then(|d| d.and_hms_opt(12, 0, 0))
                        .map(|naive| naive.and_utc());
                    let mut builder = ProcedureBuilder::default();
                    builder
                        .id(Uuid::new_v4().to_string())
                        .status(ProcedureStatus::Completed)
                        .code(CodeableConcept {
                            text: "Última visita al dentista".to_string(),
                            coding: None,
                        })
                        .subject(subject());
                    if let Some(p) = performed {
                        builder.performed_date_time(p);
                    }
                    if !visit_note.trim().is_empty() {
                        builder.note(vec![salud_types::Annotation {
                            time: Some(Utc::now()),
                            text: visit_note.clone(),
                        }]);
                    }
                    if let Ok(procedure) = builder.build() {
                        if let Err(e) = procedure_store.save(&procedure, &keypair).await {
                            log!("Error saving procedure:", format!("{:?}", e));
                        }
                    }
                }

                is_saving.set(false);
                navigator.push(&crate::router::Route::PatientDetail { id: patient_id });
            });
        })
    };

    let handle_cancel = {
        let navigator = navigator;
        let id = props.patient_id.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&crate::router::Route::PatientDetail { id: id.clone() });
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
                        {"Volver al Paciente"}
                    </button>
                    <Title>{"Ficha Médica Inicial"}</Title>
                    <MutedText>{"Información clínica básica del paciente"}</MutedText>
                </div>

                if let Some(err) = (*error).as_ref() {
                    <div class="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg text-sm text-red-700">
                        {err}
                    </div>
                }

                <div class="flex flex-col gap-4">
                    // 1. Age (from patient record)
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="p-6">
                            <Subtitle class="mb-1">{"Edad"}</Subtitle>
                            <MutedText class="mb-2">{"Calculada desde la fecha de nacimiento del paciente."}</MutedText>
                            <p class="text-lg font-semibold text-foreground">
                                {match patient_age {
                                    Some(age) => format!("{age} años"),
                                    None => "Sin fecha de nacimiento registrada".to_string(),
                                }}
                            </p>
                        </div>
                    </shady_minions::ui::Card>

                    // 2. Reason for consult
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="p-6">
                            <Subtitle class="mb-2">{"Motivo de Consulta"}</Subtitle>
                            <textarea
                                value={(*reason).clone()}
                                oninput={
                                    let reason = reason.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                        reason.set(input.value());
                                    })
                                }
                                rows="2"
                                placeholder="Ej: dolor de muela, revisión general, sangrado de encías..."
                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                            />
                        </div>
                    </shady_minions::ui::Card>

                    // 3. Last dental visit
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="p-6">
                            <Subtitle class="mb-2">{"Última Visita al Dentista"}</Subtitle>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <Label class="block mb-2">{"Fecha aproximada"}</Label>
                                    <input
                                        type="date"
                                        value={(*last_visit_date).clone()}
                                        oninput={
                                            let last_visit_date = last_visit_date.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                last_visit_date.set(input.value());
                                            })
                                        }
                                        class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    />
                                </div>
                                <div>
                                    <Label class="block mb-2">{"Detalle (opcional)"}</Label>
                                    <input
                                        type="text"
                                        value={(*last_visit_note).clone()}
                                        oninput={
                                            let last_visit_note = last_visit_note.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                last_visit_note.set(input.value());
                                            })
                                        }
                                        placeholder="Ej: limpieza hace 1 año"
                                        class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    />
                                </div>
                            </div>
                        </div>
                    </shady_minions::ui::Card>

                    // 4. Allergies
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="p-6">
                            <Subtitle class="mb-2">{"Alergias"}</Subtitle>
                            <MutedText class="mb-3">{"Marca las que apliquen y agrega otras si es necesario."}</MutedText>
                            <div class="grid grid-cols-2 md:grid-cols-3 gap-2 mb-3">
                                { for COMMON_ALLERGIES.iter().map(|item| {
                                    let item = (*item).to_string();
                                    let checked = allergies.contains(&item);
                                    html! {
                                        <button
                                            type="button"
                                            onclick={toggle_set(allergies.clone(), item.clone())}
                                            class={classes!(
                                                "text-left", "px-3", "py-2", "rounded-lg", "border", "text-sm", "transition-colors",
                                                if checked { "bg-primary/10 border-primary text-foreground font-medium" } else { "border-muted text-muted hover:bg-muted/20" }
                                            )}
                                        >
                                            {if checked { "✓ " } else { "" }}{item}
                                        </button>
                                    }
                                })}
                            </div>
                            <Label class="block mb-2">{"Otras alergias"}</Label>
                            <input
                                type="text"
                                value={(*allergy_other).clone()}
                                oninput={
                                    let allergy_other = allergy_other.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        allergy_other.set(input.value());
                                    })
                                }
                                placeholder="Separadas por comas"
                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                            />
                        </div>
                    </shady_minions::ui::Card>

                    // 5. Systemic conditions
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="p-6">
                            <Subtitle class="mb-2">{"Problemas Sistémicos"}</Subtitle>
                            <MutedText class="mb-3">{"Condiciones médicas relevantes para el tratamiento dental."}</MutedText>
                            <div class="grid grid-cols-2 md:grid-cols-3 gap-2 mb-3">
                                { for COMMON_CONDITIONS.iter().map(|item| {
                                    let item = (*item).to_string();
                                    let checked = conditions.contains(&item);
                                    html! {
                                        <button
                                            type="button"
                                            onclick={toggle_set(conditions.clone(), item.clone())}
                                            class={classes!(
                                                "text-left", "px-3", "py-2", "rounded-lg", "border", "text-sm", "transition-colors",
                                                if checked { "bg-primary/10 border-primary text-foreground font-medium" } else { "border-muted text-muted hover:bg-muted/20" }
                                            )}
                                        >
                                            {if checked { "✓ " } else { "" }}{item}
                                        </button>
                                    }
                                })}
                            </div>
                            <Label class="block mb-2">{"Otros problemas sistémicos"}</Label>
                            <input
                                type="text"
                                value={(*condition_other).clone()}
                                oninput={
                                    let condition_other = condition_other.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        condition_other.set(input.value());
                                    })
                                }
                                placeholder="Separados por comas"
                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                            />
                        </div>
                    </shady_minions::ui::Card>

                    // Actions
                    <div class="flex justify-end gap-3 pt-2">
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
                            onclick={Some(handle_save)}
                            loading={*is_saving}
                            disabled={*is_saving}
                        >
                            if !*is_saving {
                                <crate::components::Check class="size-5" />
                            }
                            {if *is_saving { "Guardando..." } else { "Guardar Ficha" }}
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}
