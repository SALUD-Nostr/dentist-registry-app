//! Encounter scheduling form - multi-step

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use gloo_console::log;
use salud_types::{
    CodeableConcept, EncounterBuilder, EncounterClass, EncounterStatus, Patient, Period, Reference,
};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typography::{Title, Subtitle, Label, MutedText, NormalText};

#[derive(Clone, PartialEq)]
enum FormStep {
    SelectPatient,
    SelectDateTime,
    Confirmation,
}

#[function_component(EncounterForm)]
pub fn encounter_form() -> Html {
    let navigator = use_navigator().unwrap();
    let patient_store = crate::storage::use_patient_store();
    let encounter_store = crate::storage::use_encounter_store();
    let notify_encounters_changed = crate::storage::use_notify_encounters_changed();
    let nostr_key = nostr_minions::use_nostr_key();

    // Form state
    let current_step = use_state(|| FormStep::SelectPatient);
    let selected_patient = use_state(|| None::<Patient>);
    let patient_search_query = use_state(String::new);
    let search_results = use_state(|| Vec::<Patient>::new());
    let is_searching = use_state(|| false);

    // Date/time state
    let appointment_date = use_state(|| {
        // Default to tomorrow
        let tomorrow = Local::now().date_naive() + chrono::Duration::days(1);
        tomorrow.format("%Y-%m-%d").to_string()
    });
    let appointment_time = use_state(|| "09:00".to_string());
    let appointment_duration = use_state(|| 30); // minutes
    let appointment_class = use_state(|| EncounterClass::Ambulatory);
    let appointment_reason = use_state(String::new);

    // Submission state
    let is_saving = use_state(|| false);
    let errors = use_state(|| Vec::<String>::new());

    // Search patients
    {
        let patient_search_query = patient_search_query.clone();
        let search_results = search_results.clone();
        let is_searching = is_searching.clone();
        let patient_store = patient_store.clone();

        use_effect_with((*patient_search_query).clone(), move |query| {
            if query.trim().len() >= 2 {
                is_searching.set(true);
                let query = query.clone();

                spawn_local(async move {
                    match patient_store.search(&query).await {
                        Ok(results) => {
                            log!("Search results:", format!("{} patients", results.len()));
                            search_results.set(results);
                            is_searching.set(false);
                        }
                        Err(e) => {
                            log!("Error searching patients:", format!("{:?}", e));
                            is_searching.set(false);
                        }
                    }
                });
            } else {
                search_results.set(Vec::new());
            }

            || ()
        });
    }

    let handle_patient_select = {
        let selected_patient = selected_patient.clone();
        let current_step = current_step.clone();
        let patient_search_query = patient_search_query.clone();
        let search_results = search_results.clone();

        Callback::from(move |patient: Patient| {
            selected_patient.set(Some(patient));
            current_step.set(FormStep::SelectDateTime);
            patient_search_query.set(String::new());
            search_results.set(Vec::new());
        })
    };

    let handle_next_to_confirmation = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            current_step.set(FormStep::Confirmation);
        })
    };

    let handle_back_to_patient = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            current_step.set(FormStep::SelectPatient);
        })
    };

    let handle_back_to_datetime = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            current_step.set(FormStep::SelectDateTime);
        })
    };

    let handle_submit = {
        let navigator = navigator.clone();
        let encounter_store = encounter_store.clone();
        let selected_patient = selected_patient.clone();
        let appointment_date = appointment_date.clone();
        let appointment_time = appointment_time.clone();
        let appointment_duration = appointment_duration.clone();
        let appointment_class = appointment_class.clone();
        let appointment_reason = appointment_reason.clone();
        let is_saving = is_saving.clone();
        let errors = errors.clone();
        let notify_encounters_changed = notify_encounters_changed.clone();
        let nostr_key = nostr_key.clone();

        Callback::from(move |_| {
            let Some(patient) = (*selected_patient).as_ref() else {
                errors.set(vec!["No se ha seleccionado un paciente".to_string()]);
                return;
            };

            let Some(patient_id) = &patient.id else {
                errors.set(vec!["El paciente no tiene ID".to_string()]);
                return;
            };

            is_saving.set(true);
            errors.set(Vec::new());

            let navigator = navigator.clone();
            let encounter_store = encounter_store.clone();
            let patient_id = patient_id.clone();
            let date_str = (*appointment_date).clone();
            let time_str = (*appointment_time).clone();
            let duration = *appointment_duration;
            let class = (*appointment_class).clone();
            let reason = (*appointment_reason).clone();
            let is_saving = is_saving.clone();
            let errors = errors.clone();
            let notify_encounters_changed = notify_encounters_changed.clone();
            let nostr_key = nostr_key.clone();

            spawn_local(async move {
                // Parse date and time
                let date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                    Ok(d) => d,
                    Err(e) => {
                        log!("Error parsing date:", format!("{:?}", e));
                        errors.set(vec!["Fecha inválida".to_string()]);
                        is_saving.set(false);
                        return;
                    }
                };

                let time = match NaiveTime::parse_from_str(&time_str, "%H:%M") {
                    Ok(t) => t,
                    Err(e) => {
                        log!("Error parsing time:", format!("{:?}", e));
                        errors.set(vec!["Hora inválida".to_string()]);
                        is_saving.set(false);
                        return;
                    }
                };

                let start = NaiveDateTime::new(date, time);
                let end = start + chrono::Duration::minutes(duration.into());

                // Build Period
                let period = Period {
                    start: Some(start.and_utc()),
                    end: Some(end.and_utc()),
                };

                // Build Reference
                let patient_ref = Reference {
                    reference: Some(format!("Patient/{}", patient_id)),
                    type_: Some("Patient".to_string()),
                    display: None,
                };

                // Build Encounter
                let mut builder = EncounterBuilder::default();
                builder
                    .id(Uuid::new_v4().to_string())
                    .resource_type("Encounter".to_string())
                    .status(EncounterStatus::Planned)
                    .class(class)
                    .subject(patient_ref)
                    .period(period);

                if !reason.trim().is_empty() {
                    builder.reason_code(vec![CodeableConcept {
                        text: reason.clone(),
                        coding: None,
                    }]);
                }

                let encounter = match builder.build() {
                    Ok(e) => e,
                    Err(e) => {
                        log!("Error building encounter:", format!("{:?}", e));
                        errors.set(vec![format!("Error al crear cita: {}", e)]);
                        is_saving.set(false);
                        return;
                    }
                };

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

                log!("Saving encounter:", format!("{:?}", encounter));
                match encounter_store.save(&encounter, keypair).await {
                    Ok(()) => {
                        log!("Encounter saved successfully");
                        notify_encounters_changed.emit(());
                        navigator.push(&crate::router::Route::EncountersSchedule);
                    }
                    Err(e) => {
                        log!("Error saving encounter:", format!("{:?}", e));
                        errors.set(vec![format!("Error al guardar: {:?}", e)]);
                        is_saving.set(false);
                    }
                }
            });
        })
    };

    let handle_cancel = {
        let navigator = navigator;
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
                    <Title>{"Agendar Nueva Cita"}</Title>
                </div>

                // Error messages
                if !(*errors).is_empty() {
                    <div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
                        <div class="flex items-start gap-2">
                            <crate::components::X class="size-5 text-red-600 mt-0.5" />
                            <div class="flex-1">
                                <Subtitle class="font-semibold text-red-900 mb-1">{"Errores"}</Subtitle>
                                <ul class="list-disc list-inside text-sm text-red-700">
                                    { for (*errors).iter().map(|error| html! {
                                        <li>{error}</li>
                                    }) }
                                </ul>
                            </div>
                        </div>
                    </div>
                }

                // Progress indicator
                <div class="mb-8">
                    <div class="flex items-center justify-center gap-2">
                        // Step 1
                        <div class="flex items-center gap-2">
                            <div class={classes!(
                                "size-8",
                                "rounded-full",
                                "flex",
                                "items-center",
                                "justify-center",
                                "font-semibold",
                                "text-sm",
                                if *current_step == FormStep::SelectPatient {
                                    "bg-primary text-white"
                                } else {
                                    "bg-green-500 text-white"
                                }
                            )}>
                                if *current_step == FormStep::SelectPatient {
                                    {"1"}
                                } else {
                                    <crate::components::Check class="size-4" />
                                }
                            </div>
                            <span class={classes!(
                                "text-sm",
                                "font-medium",
                                if *current_step == FormStep::SelectPatient {
                                    "text-foreground"
                                } else {
                                    "text-muted"
                                }
                            )}>
                                {"Paciente"}
                            </span>
                        </div>

                        <div class="w-12 h-0.5 bg-muted"></div>

                        // Step 2
                        <div class="flex items-center gap-2">
                            <div class={classes!(
                                "size-8",
                                "rounded-full",
                                "flex",
                                "items-center",
                                "justify-center",
                                "font-semibold",
                                "text-sm",
                                match *current_step {
                                    FormStep::SelectDateTime => "bg-primary text-white",
                                    FormStep::Confirmation => "bg-green-500 text-white",
                                    _ => "bg-muted text-muted-foreground"
                                }
                            )}>
                                {
                                    match *current_step {
                                        FormStep::Confirmation => html! { <crate::components::Check class="size-4" /> },
                                        _ => html! { {"2"} }
                                    }
                                }
                            </div>
                            <span class={classes!(
                                "text-sm",
                                "font-medium",
                                if *current_step == FormStep::SelectDateTime {
                                    "text-foreground"
                                } else {
                                    "text-muted"
                                }
                            )}>
                                {"Fecha y Hora"}
                            </span>
                        </div>

                        <div class="w-12 h-0.5 bg-muted"></div>

                        // Step 3
                        <div class="flex items-center gap-2">
                            <div class={classes!(
                                "size-8",
                                "rounded-full",
                                "flex",
                                "items-center",
                                "justify-center",
                                "font-semibold",
                                "text-sm",
                                if *current_step == FormStep::Confirmation {
                                    "bg-primary text-white"
                                } else {
                                    "bg-muted text-muted-foreground"
                                }
                            )}>
                                {"3"}
                            </div>
                            <span class={classes!(
                                "text-sm",
                                "font-medium",
                                if *current_step == FormStep::Confirmation {
                                    "text-foreground"
                                } else {
                                    "text-muted"
                                }
                            )}>
                                {"Confirmar"}
                            </span>
                        </div>
                    </div>
                </div>

                // Step content
                <shady_minions::ui::Card class="!border-0 !shadow-none">
                    if *current_step == FormStep::SelectPatient {
                        // Step 1: Patient Search
                        <div class="space-y-6">
                            <div>
                                <Subtitle class="mb-4">{"Seleccionar Paciente"}</Subtitle>
                                <MutedText class="mb-4">
                                    {"Busca al paciente por nombre para agendar la cita"}
                                </MutedText>
                            </div>

                            <div class="relative">
                                <div class="relative">
                                    <input
                                        type="text"
                                        placeholder="Buscar paciente por nombre..."
                                        value={(*patient_search_query).clone()}
                                        oninput={
                                            let patient_search_query = patient_search_query.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                patient_search_query.set(input.value());
                                            })
                                        }
                                        class="w-full px-4 py-3 pl-10 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    />
                                    <crate::components::Search class="absolute left-3 top-1/2 -translate-y-1/2 size-5 text-muted" />
                                </div>

                                // Search results dropdown
                                if !(*search_results).is_empty() || *is_searching {
                                    <div class="absolute z-10 w-full mt-2 bg-white border border-muted rounded-lg shadow-lg max-h-96 overflow-auto">
                                        if *is_searching {
                                            <div class="p-4 text-center text-muted">
                                                <div class="size-6 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-2"></div>
                                                {"Buscando..."}
                                            </div>
                                        } else {
                                            { for (*search_results).iter().map(|patient| {
                                                let patient_clone = patient.clone();
                                                let handle_patient_select = handle_patient_select.clone();
                                                html! {
                                                    <button
                                                        onclick={Callback::from(move |_| {
                                                            handle_patient_select.emit(patient_clone.clone());
                                                        })}
                                                        class="w-full px-4 py-3 text-left hover:bg-primary/5 transition-colors border-b border-muted/20 last:border-0"
                                                    >
                                                        <div class="flex items-center gap-3">
                                                            <crate::components::User class="size-5 text-muted" />
                                                            <div class="flex-1">
                                                                <NormalText>
                                                                    {patient.full_name().unwrap_or_else(|| "Sin nombre".to_string())}
                                                                </NormalText>
                                                                if let Some(birth_date) = patient.birth_date {
                                                                    <MutedText>
                                                                        {format!("Nacimiento: {}", birth_date.format("%d/%m/%Y"))}
                                                                    </MutedText>
                                                                }
                                                            </div>
                                                        </div>
                                                    </button>
                                                }
                                            }) }
                                        }
                                    </div>
                                }
                            </div>

                            if let Some(patient) = (*selected_patient).as_ref() {
                                <div class="p-4 bg-green-50 border border-green-200 rounded-lg">
                                    <div class="flex items-center gap-3">
                                        <crate::components::Check class="size-5 text-green-600" />
                                        <div>
                                            <NormalText class="text-green-900">
                                                {"Paciente seleccionado: "}
                                                {patient.full_name().unwrap_or_else(|| "Sin nombre".to_string())}
                                            </NormalText>
                                        </div>
                                    </div>
                                </div>
                            }

                            <div class="flex justify-end gap-3 pt-4 border-t border-muted">
                                <button
                                    onclick={handle_cancel.clone()}
                                    class="px-4 py-2 border border-muted text-foreground rounded-lg hover:bg-muted/10 transition-colors"
                                >
                                    {"Cancelar"}
                                </button>
                                <button
                                    onclick={handle_next_to_confirmation.clone()}
                                    disabled={(*selected_patient).is_none()}
                                    class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                                >
                                    {"Siguiente"}
                                    <svg class="size-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    } else if *current_step == FormStep::SelectDateTime {
                        // Step 2: Date/Time and Type Selection
                        <div class="space-y-6">
                            <div>
                                <Subtitle class="mb-4">{"Fecha, Hora y Tipo de Cita"}</Subtitle>
                                <MutedText class="mb-4">
                                    {"Selecciona cuándo será la cita y qué tipo de consulta"}
                                </MutedText>
                            </div>

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                // Date
                                <div>
                                    <Label class="mb-2">
                                        {"Fecha"}<span class="text-red-500">{"*"}</span>
                                    </Label>
                                    <input
                                        type="date"
                                        value={(*appointment_date).clone()}
                                        oninput={
                                            let appointment_date = appointment_date.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                appointment_date.set(input.value());
                                            })
                                        }
                                        class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    />
                                </div>

                                // Time
                                <div>
                                    <Label class="mb-2">
                                        {"Hora"}<span class="text-red-500">{"*"}</span>
                                    </Label>
                                    <input
                                        type="time"
                                        value={(*appointment_time).clone()}
                                        oninput={
                                            let appointment_time = appointment_time.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                appointment_time.set(input.value());
                                            })
                                        }
                                        class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    />
                                </div>

                                // Duration
                                <div>
                                    <Label class="mb-2">
                                        {"Duración (minutos)"}
                                    </Label>
                                    <select
                                        value={(*appointment_duration).to_string()}
                                        onchange={
                                            let appointment_duration = appointment_duration.clone();
                                            Callback::from(move |e: Event| {
                                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                                if let Ok(val) = select.value().parse::<i64>() {
                                                    appointment_duration.set(val);
                                                }
                                            })
                                        }
                                        class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    >
                                        <option value="15">{"15 minutos"}</option>
                                        <option value="30">{"30 minutos"}</option>
                                        <option value="45">{"45 minutos"}</option>
                                        <option value="60">{"1 hora"}</option>
                                        <option value="90">{"1.5 horas"}</option>
                                        <option value="120">{"2 horas"}</option>
                                    </select>
                                </div>

                                // Encounter Class
                                <div>
                                    <Label class="mb-2">
                                        {"Tipo de Cita"}<span class="text-red-500">{"*"}</span>
                                    </Label>
                                    <select
                                        value={match *appointment_class {
                                            EncounterClass::Ambulatory => "ambulatory",
                                            EncounterClass::Emergency => "emergency",
                                            EncounterClass::HomeHealth => "homehealth",
                                            EncounterClass::Virtual => "virtual",
                                            _ => "ambulatory",
                                        }}
                                        onchange={
                                            let appointment_class = appointment_class.clone();
                                            Callback::from(move |e: Event| {
                                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                                let class = match select.value().as_str() {
                                                    "ambulatory" => EncounterClass::Ambulatory,
                                                    "emergency" => EncounterClass::Emergency,
                                                    "homehealth" => EncounterClass::HomeHealth,
                                                    "virtual" => EncounterClass::Virtual,
                                                    _ => EncounterClass::Ambulatory,
                                                };
                                                appointment_class.set(class);
                                            })
                                        }
                                        class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                    >
                                        <option value="ambulatory">{"Consulta Ambulatoria"}</option>
                                        <option value="emergency">{"Emergencia"}</option>
                                        <option value="homehealth">{"Visita a Domicilio"}</option>
                                        <option value="virtual">{"Consulta Virtual"}</option>
                                    </select>
                                </div>
                            </div>

                            // Reason
                            <div>
                                <Label class="mb-2">
                                    {"Motivo de la Cita (Opcional)"}
                                </Label>
                                <textarea
                                    value={(*appointment_reason).clone()}
                                    oninput={
                                        let appointment_reason = appointment_reason.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                            appointment_reason.set(input.value());
                                        })
                                    }
                                    rows="3"
                                    placeholder="Ej: Revisión general, dolor de muelas, etc."
                                    class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                                />
                            </div>

                            <div class="flex justify-between gap-3 pt-4 border-t border-muted">
                                <button
                                    onclick={handle_back_to_patient}
                                    class="px-4 py-2 border border-muted text-foreground rounded-lg hover:bg-muted/10 transition-colors flex items-center gap-2"
                                >
                                    <svg class="size-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                    </svg>
                                    {"Anterior"}
                                </button>
                                <button
                                    onclick={handle_next_to_confirmation}
                                    class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors flex items-center gap-2"
                                >
                                    {"Siguiente"}
                                    <svg class="size-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    } else {
                        // Step 3: Confirmation
                        <div class="space-y-6">
                            <div>
                                <Subtitle class="mb-4">{"Confirmar Cita"}</Subtitle>
                                <MutedText class="mb-4">
                                    {"Revisa los detalles antes de agendar"}
                                </MutedText>
                            </div>

                            if let Some(patient) = (*selected_patient).as_ref() {
                                <div class="space-y-4">
                                    // Patient info
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <Subtitle class="text-muted mb-2">{"Paciente"}</Subtitle>
                                        <div class="flex items-center gap-2">
                                            <crate::components::User class="size-5 text-primary" />
                                            <NormalText>
                                                {patient.full_name().unwrap_or_else(|| "Sin nombre".to_string())}
                                            </NormalText>
                                        </div>
                                    </div>

                                    // Date and time
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <Subtitle class="text-muted mb-3">{"Fecha y Hora"}</Subtitle>
                                        <div class="grid grid-cols-2 gap-4">
                                            <div>
                                                <MutedText class="text-xs mb-1">{"Fecha"}</MutedText>
                                                <NormalText>
                                                    {
                                                        match NaiveDate::parse_from_str(&*appointment_date, "%Y-%m-%d") {
                                                            Ok(date) => date.format("%d/%m/%Y").to_string(),
                                                            Err(_) => (*appointment_date).clone()
                                                        }
                                                    }
                                                </NormalText>
                                            </div>
                                            <div>
                                                <MutedText class="text-xs mb-1">{"Hora"}</MutedText>
                                                <NormalText>{(*appointment_time).clone()}</NormalText>
                                            </div>
                                            <div>
                                                <MutedText class="text-xs mb-1">{"Duración"}</MutedText>
                                                <NormalText>{format!("{} min", *appointment_duration)}</NormalText>
                                            </div>
                                            <div>
                                                <MutedText class="text-xs mb-1">{"Tipo"}</MutedText>
                                                <NormalText>
                                                    {match *appointment_class {
                                                        EncounterClass::Emergency => "Emergencia",
                                                        EncounterClass::HomeHealth => "Visita a Domicilio",
                                                        EncounterClass::Virtual => "Consulta Virtual",
                                                        _ => "Consulta Ambulatoria",
                                                    }}
                                                </NormalText>
                                            </div>
                                        </div>
                                    </div>

                                    // Reason if provided
                                    if !(*appointment_reason).trim().is_empty() {
                                        <div class="p-4 bg-gray-50 rounded-lg">
                                            <Subtitle class="text-muted mb-2">{"Motivo"}</Subtitle>
                                            <NormalText class="text-foreground">{(*appointment_reason).clone()}</NormalText>
                                        </div>
                                    }
                                </div>
                            }

                            <div class="flex justify-between gap-3 pt-4 border-t border-muted">
                                <button
                                    onclick={handle_back_to_datetime}
                                    disabled={*is_saving}
                                    class="px-4 py-2 border border-muted text-foreground rounded-lg hover:bg-muted/10 transition-colors flex items-center gap-2 disabled:opacity-50"
                                >
                                    <svg class="size-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                    </svg>
                                    {"Anterior"}
                                </button>
                                <button
                                    onclick={handle_submit}
                                    disabled={*is_saving}
                                    class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                                >
                                    if *is_saving {
                                        <>
                                            <div class="size-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                                            {"Agendando..."}
                                        </>
                                    } else {
                                        <>
                                            <crate::components::Check class="size-5" />
                                            {"Agendar Cita"}
                                        </>
                                    }
                                </button>
                            </div>
                        </div>
                    }
                </shady_minions::ui::Card>
            </div>
        </div>
    }
}
