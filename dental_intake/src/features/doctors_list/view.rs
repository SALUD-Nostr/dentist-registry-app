use yew::prelude::*;

/// Outer wrapper that conditionally mounts the doctors list only when route is active
#[function_component(DoctorsListView)]
pub fn doctors_list_view() -> Html {
    let is_active = crate::router::use_is_route_active(crate::router::Route::DoctorsList);

    if is_active {
        html! { <DoctorsListViewInner /> }
    } else {
        // Return empty div to maintain layout, actual list won't mount/load
        html! { <div class="size-full" /> }
    }
}

/// Inner component that only mounts when `DoctorsList` route is active
#[function_component(DoctorsListViewInner)]
fn doctors_list_view_inner() -> Html {
    let doctors_list = super::provider::use_doctors_list();
    super::provider::use_load_doctors();

    let current_page = doctors_list.current_page;
    let page_doctors = &doctors_list.current_page_doctors;
    let total_pages = super::provider::total_pages(doctors_list.total_count);

    let on_search_input = {
        let doctors_list = doctors_list.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            doctors_list.dispatch(super::provider::DoctorsListAction::SetSearchQuery(
                input.value(),
            ));
        })
    };

    let do_search = {
        let doctors_list = doctors_list.clone();
        move || {
            let query = doctors_list.search_query.clone();
            doctors_list.dispatch(super::provider::DoctorsListAction::ApplySearch(query));
        }
    };

    let on_search_click = {
        let do_search = do_search.clone();
        Callback::from(move |_: MouseEvent| {
            do_search();
        })
    };

    let on_search_enter = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                do_search();
            }
        })
    };

    let on_clear_search = {
        let doctors_list = doctors_list.clone();
        Callback::from(move |_: MouseEvent| {
            doctors_list.dispatch(super::provider::DoctorsListAction::SetSearchQuery(
                String::new(),
            ));
            doctors_list.dispatch(super::provider::DoctorsListAction::ApplySearch(
                String::new(),
            ));
        })
    };

    let on_prev_page = {
        let doctors_list = doctors_list.clone();
        Callback::from(move |_| {
            if doctors_list.current_page > 0 {
                doctors_list.dispatch(super::provider::DoctorsListAction::SetPage(
                    doctors_list.current_page - 1,
                ));
            }
        })
    };

    let on_next_page = {
        let doctors_list = doctors_list.clone();
        Callback::from(move |_| {
            if doctors_list.current_page < total_pages.saturating_sub(1) {
                doctors_list.dispatch(super::provider::DoctorsListAction::SetPage(
                    doctors_list.current_page + 1,
                ));
            }
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            <div class="flex flex-col gap-4 justify-between">
                <div class="flex items-center justify-between gap-4">
                    <paravida_components::typography::Highlight>{"Doctores Registrados"}</paravida_components::typography::Highlight>
                    <crate::shared::SyncStatus />
                </div>
                <div class="flex flex-row items-center gap-2">
                    <input
                        type="text"
                        placeholder="Buscar por nombre o especialidad..."
                        value={doctors_list.search_query.clone()}
                        oninput={on_search_input}
                        onkeypress={on_search_enter}
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
                    <button
                        onclick={on_search_click}
                        class={classes!(
                            "px-4",
                            "py-2",
                            "bg-primary",
                            "text-white",
                            "rounded-lg",
                            "hover:bg-primary/90",
                            "transition-colors",
                            "text-sm",
                            "font-medium",
                            "whitespace-nowrap",
                        )}
                    >
                        {"Buscar"}
                    </button>
                    if !doctors_list.active_search.is_empty() {
                        <button
                            onclick={on_clear_search}
                            class={classes!(
                                "px-4",
                                "py-2",
                                "border",
                                "border-muted",
                                "text-foreground",
                                "rounded-lg",
                                "hover:bg-muted/10",
                                "transition-colors",
                                "text-sm",
                                "font-medium",
                                "whitespace-nowrap",
                            )}
                        >
                            {"Limpiar"}
                        </button>
                    }
                    <paravida_components::typography::P class="text-muted text-nowrap">
                        {format!("{} doctores", doctors_list.total_count)}
                    </paravida_components::typography::P>
                </div>
            </div>

            <div class="flex-1">
                <div class="bg-white rounded-lg shadow-lg border border-muted/30 overflow-hidden">
                    // Header
                    <div class="grid grid-cols-9 gap-4 bg-primary/10 px-6 py-3 font-semibold text-sm text-foreground border-b border-muted/30 sticky top-0 z-10">
                        <div class="col-span-5">{"Nombre"}</div>
                        <div class="col-span-4">{"Especialidad"}</div>
                    </div>
                    if doctors_list.loading {
                        {for (0..10).map(|_| {
                            html! {
                                <div
                                    class="grid grid-cols-9 gap-4 px-6 py-4 hover:bg-primary/5 transition-colors max-h-[calc(80vh-14rem)] overflow-y-auto"
                                >
                                    <div class="col-span-5 flex items-center">
                                        <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-full" />
                                    </div>
                                    <div class="col-span-4 flex items-center">
                                        <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-full" />
                                    </div>
                                </div>
                            }
                        })}
                    } else if doctors_list.current_page_doctors.is_empty() {
                        <paravida_components::Card class="!size-fit !flex-grow-0">
                            <div class="flex flex-col items-center justify-center gap-4 py-8">
                                <paravida_components::typography::P class="text-muted text-center">
                                    {if doctors_list.active_search.is_empty() {
                                        "No hay doctores registrados en el sistema."
                                    } else {
                                        "No se encontraron doctores con ese criterio de búsqueda."
                                    }}
                                </paravida_components::typography::P>
                            </div>
                        </paravida_components::Card>
                    } else {
                        // Rows
                        <div class="divide-y divide-muted/20  max-h-[calc(84vh-14rem)] sm:max-h-[calc(90vh-14rem)] overflow-y-auto">
                            { for page_doctors.iter().map(|doctor| {
                                let specialty = doctor
                                    .practitioner
                                    .specialty
                                    .first().map_or_else(|| "Sin especialidad".to_string(), paravida_models::ParavidaSpecialty::display_text);

                                let name = if doctor.practitioner.name.text().trim().is_empty() {
                                    "Sin nombre".to_string()
                                } else {
                                    doctor.practitioner.name.text()
                                };

                                html! {
                                    <yew_router::components::Link<crate::router::Route>
                                        to={crate::router::Route::DoctorDetail { pubkey: doctor.pubkey.clone() }}
                                        classes="grid grid-cols-9 gap-4 px-3 py-2 sm:px-6 sm:py-4 hover:bg-primary/5 transition-colors cursor-pointer hover:border-l-4 hover:border-primary"
                                    >
                                        <div class="col-span-5 flex items-center max-w-[8rem] md:max-w-none min-w-0 truncate">
                                            <span class="font-medium text-foreground">{name}</span>
                                        </div>
                                        <div class="col-span-4 flex items-center">
                                            <span class="text-muted text-sm max-w-[8rem] md:max-w-none min-w-0 truncate">
                                                {specialty}</span>
                                        </div>
                                    </yew_router::components::Link<crate::router::Route>>
                                }
                            })}
                        </div>
                    }
                </div>
            </div>
            if total_pages > 1 {
                <div class="flex flex-row justify-center items-center gap-4 sticky">
                    <button
                        onclick={on_prev_page}
                        disabled={current_page == 0}
                        class={classes!(
                            "px-4",
                            "py-2",
                            "rounded-lg",
                            "border",
                            "border-muted",
                            "text-sm",
                            "font-medium",
                            "transition-colors",
                            if current_page == 0 {
                                classes!("text-muted", "cursor-not-allowed", "opacity-50")
                            } else {
                                classes!("text-foreground", "hover:bg-primary/10", "hover:border-primary")
                            }
                        )}
                    >
                        {"← Anterior"}
                    </button>

                    <paravida_components::typography::P class="text-muted">
                        {format!("Página {} de {}", current_page + 1, total_pages)}
                    </paravida_components::typography::P>

                    <button
                        onclick={on_next_page}
                        disabled={current_page >= total_pages.saturating_sub(1)}
                        class={classes!(
                            "px-4",
                            "py-2",
                            "rounded-lg",
                            "border",
                            "border-muted",
                            "text-sm",
                            "font-medium",
                            "transition-colors",
                            if current_page >= total_pages.saturating_sub(1) {
                                classes!("text-muted", "cursor-not-allowed", "opacity-50")
                            } else {
                                classes!("text-foreground", "hover:bg-primary/10", "hover:border-primary")
                            }
                        )}
                    >
                        {"Siguiente →"}
                    </button>
                </div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DoctorDetailViewProps {
    pub pubkey: String,
}

#[function_component(DoctorDetailView)]
pub fn doctor_detail_view(props: &DoctorDetailViewProps) -> Html {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();

    let doctor = yew::suspense::use_future_with(
        (props.pubkey.clone(), local_db.clone(), nostr_key.clone()),
        move |deps| {
            let (pubkey, local_db, nostr_key) = deps.as_ref().clone();
            async move {
                let local_db = local_db.as_ref()?;
                let nostr_key = nostr_key.as_ref()?;

                let note = local_db.get_doctor(&pubkey).await.ok()??;
                let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                let shared_document = mutual_note.view_shared_document(nostr_key).ok()?;
                let practitioner =
                    paravida_models::ParavidaPractitioner::from_salud_note(&shared_document)
                        .ok()?;

                Some(super::provider::DoctorListItem {
                    pubkey,
                    practitioner,
                })
            }
        },
    );

    let doctor = match doctor {
        Ok(doctor_handle) => match &*doctor_handle {
            Some(doctor) => doctor.clone(),
            None => {
                return html! {
                    <div class="flex flex-col size-full p-4 gap-6">
                        <paravida_components::Card class="!max-w-2xl mx-auto">
                            <div class="flex flex-col items-center justify-center gap-4 py-8">
                                <paravida_components::typography::P class="text-muted text-center">
                                    {"Doctor no encontrado"}
                                </paravida_components::typography::P>
                                <yew_router::components::Link<crate::router::Route>
                                    to={crate::router::Route::DoctorsList}
                                    classes="text-primary hover:underline"
                                >
                                    {"Volver a la lista"}
                                </yew_router::components::Link<crate::router::Route>>
                            </div>
                        </paravida_components::Card>
                    </div>
                };
            }
        },
        Err(_) => {
            return html! {
                <div class="flex justify-center items-center flex-1">
                    <paravida_components::icons::ParavidaLogo class="size-12 animate-pulse" />
                </div>
            };
        }
    };

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            <div class="flex flex-row items-center gap-4 mb-4">
                <yew_router::components::Link<crate::router::Route>
                    to={crate::router::Route::DoctorsList}
                    classes="text-primary hover:text-secondary flex items-center gap-2"
                >
                    <paravida_components::icons::ChevronLeft class="size-6" />
                    <span>{"Volver"}</span>
                </yew_router::components::Link<crate::router::Route>>
            </div>

            <div class="flex flex-col gap-6">
                <paravida_components::typography::Highlight>{"Detalles del Doctor"}</paravida_components::typography::Highlight>
                <super::display::DoctorCard doctor={doctor.clone()} />
            </div>
        </div>
    }
}
