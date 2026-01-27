//! Encounters history view with pagination and search

use chrono::{Datelike, Utc};
use gloo_console::log;
use salud_types::{Encounter, EncounterClass, EncounterStatus, Patient};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Button, ButtonVariant, ButtonSize};
use crate::components::typography::{MutedText, NormalText, Title};

// Pagination page size
const PAGE_SIZE: usize = 10;

#[function_component(EncountersHistory)]
pub fn encounters_history() -> Html {
    let navigator = use_navigator().unwrap();
    let encounter_store = crate::storage::use_encounter_store();
    let patient_store = crate::storage::use_patient_store();
    let encounter_version = crate::storage::use_encounter_store_version();

    // Pagination state
    let page = use_state(|| 0usize);
    let search_query = use_state(String::new);

    // Data state
    let encounters = use_state(Vec::<(Encounter, Option<Patient>)>::new);
    let total_count = use_state(|| 0usize);
    let is_loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load encounters when page or search changes
    {
        let encounters = encounters.clone();
        let total_count = total_count.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();

        use_effect_with(
            (*page, (*search_query).clone(), encounter_version),
            move |(page_num, query, _version)| {
                let query = query.to_lowercase();
                let page_num = *page_num;

                is_loading.set(true);
                error.set(None);

                let encounter_store = encounter_store.clone();
                let patient_store = patient_store.clone();
                let encounters = encounters.clone();
                let total_count = total_count.clone();
                let is_loading = is_loading.clone();

                spawn_local(async move {
                    let Ok(all_encounters) = encounter_store.get_all().await else {
                        error.set(Some(format!("Error al cargar citas",)));
                        is_loading.set(false);
                        return;
                    };

                    log!(
                        "Loaded encounters:",
                        format!("{} total", all_encounters.len())
                    );

                    // Show all encounters regardless of status
                    let mut filtered: Vec<Encounter> = all_encounters;

                    // Apply search filter
                    if !query.trim().is_empty() {
                        let filtered_with_search =
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

                        for encounter in filtered {
                            let patient_ref = encounter.subject.reference.clone();

                            if let Some(ref_str) = patient_ref {
                                if ref_str.to_lowercase().contains(&query) {
                                    filtered_with_search.borrow_mut().push(encounter);
                                    continue;
                                }

                                // Try to get patient name for search
                                if let Some(patient_id) = ref_str.strip_prefix("Patient/")
                                    && let Ok(Some(patient)) = patient_store.get(patient_id).await
                                    && let Some(full_name) = patient.full_name()
                                    && full_name.to_lowercase().contains(&query)
                                {
                                    filtered_with_search.borrow_mut().push(encounter);
                                }
                            }
                        }

                        filtered = filtered_with_search.borrow().clone();
                    }

                    // Sort by date descending (most recent first)
                    filtered.sort_by(|a, b| {
                        let date_a = a
                            .period
                            .as_ref()
                            .and_then(|p| p.start)
                            .unwrap_or_else(Utc::now);
                        let date_b = b
                            .period
                            .as_ref()
                            .and_then(|p| p.start)
                            .unwrap_or_else(Utc::now);
                        date_b.cmp(&date_a)
                    });

                    // Get paginated slice
                    let start = page_num * PAGE_SIZE;
                    let end = (start + PAGE_SIZE).min(filtered.len());
                    let paginated = if start < filtered.len() {
                        filtered[start..end].to_vec()
                    } else {
                        Vec::new()
                    };

                    // Load patient information for each encounter
                    let mut encounters_with_patients = Vec::new();
                    for encounter in paginated {
                        let patient = if let Some(ref_str) = encounter.subject.reference.as_ref() {
                            if let Some(patient_id) = ref_str.strip_prefix("Patient/") {
                                match patient_store.get(patient_id).await {
                                    Ok(Some(p)) => Some(p),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        encounters_with_patients.push((encounter, patient));
                    }

                    encounters.set(encounters_with_patients);
                    total_count.set(filtered.len());
                    is_loading.set(false);
                });

                || ()
            },
        );
    }

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let handle_encounter_click = {
        let navigator = navigator.clone();
        Callback::from(move |encounter_id: String| {
            navigator.push(&crate::router::Route::EncounterDetail { id: encounter_id });
        })
    };

    let next_page = {
        let page = page.clone();
        Callback::from(move |_: yew::MouseEvent| page.set(*page + 1))
    };

    let prev_page = {
        let page = page.clone();
        Callback::from(move |_: yew::MouseEvent| {
            if *page > 0 {
                page.set(*page - 1);
            }
        })
    };

    let total_pages = if *total_count > 0 {
        (*total_count + PAGE_SIZE - 1) / PAGE_SIZE
    } else {
        1
    };

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            // Header
            <div class="flex flex-col gap-4">
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    <Title>{"Historial de Citas"}</Title>
                    <div class="text-sm text-muted">
                        {format!("{} cita(s)", *total_count)}
                    </div>
                </div>

                // Search input
                <div class="relative">
                    <input
                        type="text"
                        placeholder="Buscar por paciente o fecha..."
                        value={(*search_query).clone()}
                        oninput={on_search_input}
                        class={classes!(
                            "w-full",
                            "border",
                            "border-muted",
                            "text-foreground",
                            "text-sm",
                            "sm:text-base",
                            "rounded-lg",
                            "py-2",
                            "pl-10",
                            "pr-4",
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
                    <crate::components::Search class="absolute left-3 top-1/2 -translate-y-1/2 size-5 text-muted" />
                </div>
            </div>

            // Table with loading state
            <div class="flex-1">
                <div class="bg-white rounded-lg shadow-lg border border-muted/30 overflow-hidden">
                    // Header
                    <div class="grid grid-cols-12 gap-2 sm:gap-4 bg-primary/10 px-3 sm:px-6 py-2 sm:py-3 font-semibold text-xs sm:text-sm text-foreground border-b border-muted/30 sticky top-0 z-10">
                        <div class="col-span-5 sm:col-span-4">{"Paciente"}</div>
                        <div class="col-span-4 sm:col-span-3 hidden sm:block">{"Fecha"}</div>
                        <div class="col-span-3 sm:col-span-2">{"Estado"}</div>
                        <div class="col-span-4 sm:col-span-3 hidden sm:block">{"Tipo"}</div>
                    </div>

                    // Content area
                    if *is_loading {
                        <EncountersHistorySkeleton />
                    } else if let Some(err) = (*error).as_ref() {
                        <div class="p-8 flex items-center justify-center">
                            <div class="flex flex-col items-center gap-4">
                                <crate::components::X class="size-16 text-red-600" />
                                <NormalText class="text-red-600 font-semibold">{err.clone()}</NormalText>
                            </div>
                        </div>
                    } else if (*encounters).is_empty() && (*search_query).is_empty() {
                        // No encounters at all
                        <div class="p-8 flex items-center justify-center">
                            <shady_minions::ui::Card class="!border-0 !shadow-none">
                                <div class="flex flex-col items-center justify-center gap-4 py-8">
                                    <crate::components::List class="size-16 text-muted opacity-50" />
                                    <NormalText class="text-muted text-center font-semibold">
                                        {"No hay citas en el historial"}
                                    </NormalText>
                                    <MutedText class="text-center">
                                        {"Las citas completadas o canceladas aparecerán aquí."}
                                    </MutedText>
                                </div>
                            </shady_minions::ui::Card>
                        </div>
                    } else if (*encounters).is_empty() {
                        // No search results
                        <div class="p-8 flex items-center justify-center">
                            <shady_minions::ui::Card class="!border-0 !shadow-none">
                                <div class="flex flex-col items-center justify-center gap-4 py-8">
                                    <crate::components::Search class="size-16 text-muted opacity-50" />
                                    <NormalText class="text-muted text-center font-semibold">
                                        {"No se encontraron citas"}
                                    </NormalText>
                                    <MutedText class="text-center">
                                        {format!("No hay resultados para '{}'", *search_query)}
                                    </MutedText>
                                </div>
                            </shady_minions::ui::Card>
                        </div>
                    } else {
                        // Encounter rows
                        <div class="divide-y divide-muted/20">
                            { for (*encounters).iter().map(|(encounter, patient)| {
                                let encounter_id = encounter.id.clone().unwrap_or_default();
                                let onclick = {
                                    let handle_encounter_click = handle_encounter_click.clone();
                                    let encounter_id = encounter_id.clone();
                                    Callback::from(move |_| {
                                        handle_encounter_click.emit(encounter_id.clone());
                                    })
                                };

                                // Format date
                                let date_display = encounter.period.as_ref().and_then(|period| period.start)
                                    .map(|start| {
                                        let weekday = match start.weekday() {
                                            chrono::Weekday::Mon => "Lun",
                                            chrono::Weekday::Tue => "Mar",
                                            chrono::Weekday::Wed => "Mié",
                                            chrono::Weekday::Thu => "Jue",
                                            chrono::Weekday::Fri => "Vie",
                                            chrono::Weekday::Sat => "Sáb",
                                            chrono::Weekday::Sun => "Dom",
                                        };
                                        let date = start.format("%d/%m/%Y").to_string();
                                        format!("{}, {}", weekday, date)
                                    })
                                    .unwrap_or_else(|| "-".to_string());

                                // Format time
                                let _time_display = encounter.period.as_ref().and_then(|period| period.start)
                                    .map(|start| start.format("%H:%M").to_string())
                                    .unwrap_or_else(|| "-".to_string());

                                // Patient name
                                let patient_name = patient
                                    .as_ref()
                                    .and_then(|p| p.full_name())
                                    .unwrap_or_else(|| "Paciente".to_string());

                                // Status badge
                                let status_badge = match &encounter.status {
                                    EncounterStatus::Planned => html! {
                                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-blue-100 text-blue-800 rounded-md text-xs font-medium">
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
                                        <span class="inline-flex items-center gap-1 px-2 py-1 bg-purple-100 text-purple-800 rounded-md text-xs font-medium">
                                            {"En Progreso"}
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
                                };

                                // Encounter type
                                let encounter_type = match &encounter.class {
                                    EncounterClass::Ambulatory => "Consulta Ambulatoria",
                                    EncounterClass::Emergency => "Emergencia",
                                    EncounterClass::HomeHealth => "Visita a Domicilio",
                                    EncounterClass::Virtual => "Consulta Virtual",
                                    EncounterClass::Field => "Terreno",
                                    EncounterClass::Inpatient => "Hospitalización",
                                    EncounterClass::Acute => "Atención Aguda",
                                };

                                html! {
                                    <div
                                        key={encounter_id.clone()}
                                        onclick={onclick}
                                        class="grid grid-cols-12 gap-2 sm:gap-4 px-3 sm:px-6 py-3 sm:py-4 hover:bg-primary/5 cursor-pointer transition-colors"
                                    >
                                        <div class="col-span-5 sm:col-span-4 flex items-center gap-1 sm:gap-2 min-w-0">
                                            <crate::components::User class="size-4 sm:size-5 text-muted shrink-0" />
                                            <span class="font-medium text-xs sm:text-base text-foreground truncate">
                                                {patient_name}
                                            </span>
                                        </div>
                                        <div class="col-span-4 sm:col-span-3 hidden sm:flex items-center text-xs sm:text-sm text-muted min-w-0">
                                            <span class="truncate">
                                                {date_display}
                                            </span>
                                        </div>
                                        <div class="col-span-7 sm:col-span-2 flex items-center justify-end sm:justify-start">
                                            {status_badge}
                                        </div>
                                        <div class="col-span-4 sm:col-span-3 hidden sm:flex items-center text-xs sm:text-sm text-muted min-w-0">
                                            <span class="truncate">
                                                {encounter_type}
                                            </span>
                                        </div>
                                    </div>
                                }
                            }) }
                        </div>
                    }
                </div>
            </div>

            // Pagination
            if *total_count > PAGE_SIZE {
                <div class="flex justify-center items-center gap-4">
                    <Button
                        variant={ButtonVariant::Outline}
                        size={ButtonSize::Medium}
                        onclick={Some(prev_page)}
                        disabled={*page == 0}
                    >
                        {"← Anterior"}
                    </Button>
                    <MutedText>
                        {format!("Página {} de {}", *page + 1, total_pages)}
                    </MutedText>
                    <Button
                        variant={ButtonVariant::Outline}
                        size={ButtonSize::Medium}
                        onclick={Some(next_page)}
                        disabled={*page >= total_pages - 1}
                    >
                        {"Siguiente →"}
                    </Button>
                </div>
            }
        </div>
    }
}

#[function_component(EncountersHistorySkeleton)]
fn encounters_history_skeleton() -> Html {
    html! {
        <div class="divide-y divide-muted/20">
            {for (0..PAGE_SIZE).map(|i| {
                html! {
                    <div
                        key={format!("skeleton-{}", i)}
                        class="grid grid-cols-12 gap-4 px-6 py-4"
                    >
                        <div class="col-span-4 flex items-center gap-2">
                            <span class="bg-muted/50 animate-pulse rounded-full size-5 shrink-0" />
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-3/4" />
                        </div>
                        <div class="col-span-3 flex items-center">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-full" />
                        </div>
                        <div class="col-span-2 flex items-center">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-6 w-2/3" />
                        </div>
                        <div class="col-span-3 flex items-center">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-1/2" />
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
