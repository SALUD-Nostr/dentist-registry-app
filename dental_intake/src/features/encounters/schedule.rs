//! Encounters schedule (calendar view)

use chrono::{DateTime, Utc};
use gloo_console::log;
use salud_types::EncounterStatus;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(EncountersSchedule)]
pub fn encounters_schedule() -> Html {
    let navigator = use_navigator().unwrap();
    let encounter_store = crate::storage::use_encounter_store();
    let patient_store = crate::storage::use_patient_store();

    let calendar_state = use_mut_ref(|| None::<yew_full_calendar::Calendar>);
    let is_loading = use_state(|| false);

    let handle_new_encounter = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::EncounterNew);
        })
    };

    let on_calendar_created = {
        let calendar_state = calendar_state.clone();
        let encounter_store = encounter_store.clone();
        let patient_store = patient_store.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |cal: yew_full_calendar::Calendar| {
            calendar_state.borrow_mut().replace(cal.clone());

            if let Err(e) = load_encounters(
                calendar_state.clone(),
                Some(encounter_store.clone()),
                Some(patient_store.clone()),
                is_loading.clone(),
            ) {
                log!("Error loading encounters:", format!("{:?}", e));
            }
        })
    };

    let on_dates_set = {
        let calendar_state = calendar_state.clone();
        let encounter_store = encounter_store.clone();
        let patient_store = patient_store.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |_date_set: yew_full_calendar::DateSetEvent| {
            if let Err(e) = load_encounters(
                calendar_state.clone(),
                Some(encounter_store.clone()),
                Some(patient_store.clone()),
                is_loading.clone(),
            ) {
                log!("Error loading encounters:", format!("{:?}", e));
            }
        })
    };

    let on_event_click = {
        let navigator = navigator.clone();
        Callback::from(move |event: yew_full_calendar::EventClickInfo| {
            let Some(event) = event.event() else {
                return;
            };
            if let Some(id) = event.id() {
                navigator.push(&crate::router::Route::EncounterDetail { id });
            }
        })
    };

    let calendar_options = yew_full_calendar::Options::default()
        .with_locale(yew_full_calendar::Locale::Es)
        .with_initial_view(yew_full_calendar::InitialView::TimeGridWeek)
        .with_all_day_slot(false)
        .with_header_toolbar(yew_full_calendar::HeaderOptions::Options(
            yew_full_calendar::HeaderSections {
                left: "title".to_string(),
                center: "dayGridMonth,timeGridWeek,timeGridDay".to_string(),
                right: "today prev,next".to_string(),
            },
        ));

    html! {
        <div class="flex flex-col size-full p-4 md:p-8">
            <div class="max-w-7xl mx-auto w-full h-full flex flex-col">
                <div class="flex justify-between items-center mb-6">
                    <h1 class="text-3xl font-bold">{"Calendario de Citas"}</h1>
                    <button
                        onclick={handle_new_encounter}
                        class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 flex items-center gap-2"
                    >
                        <crate::components::Plus class="size-5" />
                        {"Nueva Cita"}
                    </button>
                </div>

                <shady_minions::ui::Card class="flex-1 !p-0 !max-w-none">
                    <style>
                        {r"
                        .fc-header-toolbar {
                            margin: 6px !important;
                        }
                        "}
                    </style>
                    <yew_full_calendar::FullCalendarComponent
                        {calendar_options}
                        calendar_id="encounters-calendar"
                        class="size-full flex-1 rounded-xl"
                        {on_calendar_created}
                        {on_dates_set}
                        {on_event_click}
                    />
                </shady_minions::ui::Card>
            </div>
        </div>
    }
}

fn load_encounters(
    calendar: std::rc::Rc<std::cell::RefCell<Option<yew_full_calendar::Calendar>>>,
    encounter_store: Option<std::rc::Rc<crate::storage::EncounterStore>>,
    patient_store: Option<std::rc::Rc<crate::storage::PatientStore>>,
    is_loading: UseStateHandle<bool>,
) -> Result<(), JsValue> {
    let Some(calendar) = calendar.borrow().as_ref().cloned() else {
        return Err(JsValue::from_str("Calendar not initialized"));
    };

    let Some(encounter_store) = encounter_store.as_ref() else {
        return Err(JsValue::from_str("Encounter store not initialized"));
    };

    let Some(patient_store) = patient_store.as_ref() else {
        return Err(JsValue::from_str("Patient store not initialized"));
    };

    let encounter_store = encounter_store.clone();
    let patient_store = patient_store.clone();

    // Get the visible date range from the calendar
    let view = calendar.view().ok_or(JsValue::from_str("Calendar view not initialized"))?;

    let start = view
        .active_start()
        .and_then(|s| {
            DateTime::parse_from_rfc3339(
                s.to_iso_string().as_string().unwrap_or_default().as_str(),
            )
            .ok()
        })
        .ok_or(JsValue::from_str("Failed to get start date"))?;

    let end = view
        .active_end()
        .and_then(|s| {
            DateTime::parse_from_rfc3339(
                s.to_iso_string().as_string().unwrap_or_default().as_str(),
            )
            .ok()
        })
        .ok_or(JsValue::from_str("Failed to get end date"))?;

    is_loading.set(true);

    let is_loading_clone = is_loading.clone();
    let is_loading_clone2 = is_loading.clone();
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = async move {
            // Load all encounters
            let all_encounters = encounter_store.get_all().await.map_err(|e| {
                log!("Error loading encounters:", format!("{:?}", e));
                JsValue::from_str(&format!("{:?}", e))
            })?;

            log!("Loaded encounters:", format!("{} total", all_encounters.len()));

            // Filter for planned encounters in the visible range
            let mut filtered_encounters = Vec::new();
            for encounter in all_encounters {
                // Only show Planned encounters (pending to complete)
                if encounter.status != EncounterStatus::Planned {
                    continue;
                }

                // Check if encounter is in the visible date range
                if let Some(period) = &encounter.period {
                    if let Some(encounter_start) = period.start {
                        let start_utc: DateTime<Utc> = encounter_start.into();
                        if start_utc >= start.with_timezone(&Utc) && start_utc <= end.with_timezone(&Utc) {
                            filtered_encounters.push(encounter);
                        }
                    }
                }
            }

            log!("Filtered encounters:", format!("{} in range and planned", filtered_encounters.len()));

            // Load patient names for the encounters
            let mut events_with_patients = Vec::new();
            for encounter in filtered_encounters {
                let patient_name = if let Some(ref_str) = &encounter.subject.reference {
                    // Extract patient ID from reference (e.g., "Patient/123")
                    let patient_id = ref_str.strip_prefix("Patient/").unwrap_or(ref_str);

                    match patient_store.get(patient_id).await {
                        Ok(Some(patient)) => patient.full_name().unwrap_or_else(|| "Paciente".to_string()),
                        _ => "Paciente".to_string(),
                    }
                } else {
                    "Paciente".to_string()
                };

                events_with_patients.push((encounter, patient_name));
            }

            // Update calendar with events
            let calendar_clone = calendar.clone();
            let function = wasm_bindgen::closure::Closure::once_into_js(move || {
                calendar_clone.clear_events();

                for (encounter, patient_name) in events_with_patients {
                    if let Some(period) = &encounter.period {
                        if let (Some(start), Some(end)) = (period.start, period.end) {
                            let title = format!("{} - {}", patient_name,
                                match &encounter.class {
                                    salud_types::EncounterClass::Ambulatory => "Consulta",
                                    salud_types::EncounterClass::Emergency => "Emergencia",
                                    salud_types::EncounterClass::HomeHealth => "Domicilio",
                                    salud_types::EncounterClass::Virtual => "Virtual",
                                    salud_types::EncounterClass::Field => "Terreno",
                                    salud_types::EncounterClass::Inpatient => "Hospitalización",
                                    salud_types::EncounterClass::Acute => "Aguda",
                                }
                            );

                            let color = match &encounter.class {
                                salud_types::EncounterClass::Ambulatory => "#3b82f6", // blue
                                salud_types::EncounterClass::Emergency => "#ef4444", // red
                                salud_types::EncounterClass::HomeHealth => "#10b981", // green
                                salud_types::EncounterClass::Virtual => "#8b5cf6", // purple
                                salud_types::EncounterClass::Field => "#f59e0b", // amber
                                salud_types::EncounterClass::Inpatient => "#06b6d4", // cyan
                                salud_types::EncounterClass::Acute => "#ec4899", // pink
                            };

                            let event = yew_full_calendar::EventBuilder::default()
                                .id(encounter.id.as_ref().unwrap_or(&String::new()).as_str())
                                .title(&title)
                                .display(yew_full_calendar::EventDisplay::Block)
                                .text_color("#ffffff")
                                .border_color(color)
                                .background_color(color)
                                .start(yew_full_calendar::EventDate::Iso8601String(
                                    start.to_rfc3339(),
                                ))
                                .end(yew_full_calendar::EventDate::Iso8601String(
                                    end.to_rfc3339(),
                                ));

                            if let Err(e) = calendar_clone.add_or_replace_event(event) {
                                log!("Error adding event:", format!("{:?}", e));
                            }
                        }
                    }
                }
            });

            calendar.batch_rendering(function.into());
            is_loading_clone.set(false);
            Ok::<(), JsValue>(())
        }
        .await
        {
            log!("Error in load_encounters:", format!("{:?}", e));
            is_loading_clone2.set(false);
        }
    });

    Ok(())
}
