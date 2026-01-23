use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryProviderProps {
    pub children: Html,
}

#[function_component(HistoryProvider)]
pub fn history_provider(props: &HistoryProviderProps) -> Html {
    let cursors = super::hooks::use_pagination_cursors();

    html! {
        <ContextProvider<UseStateHandle<super::hooks::PaginationCursors>> context={cursors}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<super::hooks::PaginationCursors>>>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct HistoryListProps {
    pub page: usize,
    pub page_size: usize,
}

#[function_component(HistoryList)]
pub fn history_list(props: &HistoryListProps) -> HtmlResult {
    let appointments = super::hooks::use_appointment_list(props.page, props.page_size)?;

    Ok(html! {
        <>
            {if appointments.is_empty() {
                html! {
                    <div class="col-span-12 py-8">
                        <paravida_components::typography::P class="text-muted text-center">
                            {"No hay citas disponibles"}
                        </paravida_components::typography::P>
                    </div>
                }
            } else {
                appointments.iter().map(|(appointment, id)| {
                    use chrono::Datelike;
                    let week_day = match appointment.start.with_timezone(&chrono::Local).weekday() {
                        chrono::Weekday::Mon => "Lun",
                        chrono::Weekday::Tue => "Mar",
                        chrono::Weekday::Wed => "Mié",
                        chrono::Weekday::Thu => "Jue",
                        chrono::Weekday::Fri => "Vie",
                        chrono::Weekday::Sat => "Sáb",
                        chrono::Weekday::Sun => "Dom",
                    };
                    let date = appointment
                        .start
                        .with_timezone(&chrono::Local)
                        .format("%d/%m/%Y")
                        .to_string();
                    let start_time = appointment
                        .start
                        .with_timezone(&chrono::Local)
                        .format("%-I:%M %p")
                        .to_string();
                    let end_time = appointment
                        .end
                        .with_timezone(&chrono::Local)
                        .format("%-I:%M %p")
                        .to_string();
                    let specialty = appointment.specialty.display_text();
                    let room = appointment.room.name();

                    html! {
                        <yew_router::components::Link<crate::router::AppRoute>
                            to={crate::router::AppRoute::AppointmentDetail { id: id.clone() }}>
                        <div
                            key={id.clone()}
                            class="grid grid-cols-12 gap-4 px-6 py-4 hover:bg-primary/5 transition-colors cursor-pointer hover:border-l-4 hover:border-primary"
                        >
                            <div class="col-span-3 text-xs sm:text-sm min-w-0">
                                <span class="block w-full truncate font-medium">
                                    {format!("{week_day}, {date}")}
                                </span>
                            </div>

                            <div class="col-span-3 text-xs sm:text-sm text-muted max-w-[4rem] md:max-w-none min-w-0">
                                <span class="block w-full truncate">
                                    {format!("{start_time} - {end_time}")}
                                </span>
                            </div>

                            <div class="col-span-3 text-xs sm:text-sm max-w-[6rem] md:max-w-none min-w-0">
                                <span class="block w-full truncate">
                                    {specialty}
                                </span>
                            </div>

                            <div class="col-span-3 text-xs sm:text-sm text-muted max-w-[5rem] md:max-w-none min-w-0">
                                <span class="block w-full truncate">
                                    {room}
                                </span>
                            </div>
                        </div>
                        </yew_router::components::Link<crate::router::AppRoute>>
                    }
                }).collect::<Html>()
            }}
        </>
    })
}

#[function_component(HistoryCount)]
pub fn history_count(props: &HistoryListProps) -> HtmlResult {
    let appointments = super::hooks::use_appointment_list(props.page, props.page_size)?;

    Ok(html! {
        <paravida_components::typography::P class="text-muted">
            {format!("Mostrando {} citas", appointments.len())}
        </paravida_components::typography::P>
    })
}

#[derive(Properties, PartialEq)]
pub struct HistoryPaginationProps {
    pub page: usize,
    pub on_next: Callback<yew::MouseEvent>,
    pub on_prev: Callback<yew::MouseEvent>,
}

#[function_component(HistoryPagination)]
pub fn history_pagination(props: &HistoryPaginationProps) -> Html {
    html! {
        <div class="flex justify-center items-center gap-4">
            <button
                onclick={props.on_prev.clone()}
                disabled={props.page == 0}
                class={classes!(
                    "px-4",
                    "py-2",
                    "rounded-lg",
                    "border",
                    "border-muted",
                    "transition-colors",
                    if props.page == 0 {
                        "opacity-50 cursor-not-allowed"
                    } else {
                        "hover:bg-primary/10"
                    }
                )}
            >
                {"← Anterior"}
            </button>
            <paravida_components::typography::P class="text-muted">
                {format!("Página {}", props.page + 1)}
            </paravida_components::typography::P>
            <button
                onclick={props.on_next.clone()}
                class="px-4 py-2 rounded-lg border border-muted hover:bg-primary/10 transition-colors"
            >
                {"Siguiente →"}
            </button>
        </div>
    }
}

#[function_component(HistoryTableSkeleton)]
pub fn history_table_skeleton() -> Html {
    html! {
        <>
            {for (0..10).map(|i| {
                html! {
                    <div
                        key={format!("skeleton-{}", i)}
                        class="grid grid-cols-12 gap-4 px-6 py-4"
                    >
                        <div class="col-span-3 flex flex-col">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-3/4" />
                        </div>
                        <div class="col-span-3 flex items-center">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-full" />
                        </div>
                        <div class="col-span-3 flex items-center">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-2/3" />
                        </div>
                        <div class="col-span-3 flex items-center">
                            <span class="bg-muted/50 animate-pulse rounded-lg h-4 w-1/2" />
                        </div>
                    </div>
                }
            })}
        </>
    }
}

#[function_component(HistoryView)]
pub fn history_view() -> Html {
    let page = use_state(|| 0usize);
    let page_size = 10usize;

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

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            // Header with suspending count
            <div class="flex flex-col gap-4">
                <div class="flex items-center justify-between gap-4">
                    <paravida_components::typography::Highlight>{"Historial de Citas"}</paravida_components::typography::Highlight>
                    <crate::shared::SyncStatus />
                </div>

                <HistoryProvider>
                    <yew::suspense::Suspense fallback={html! {
                        <paravida_components::typography::P class="text-muted">{"Cargando..."}</paravida_components::typography::P>
                    }}>
                        <HistoryCount page={*page} page_size={page_size} />
                    </yew::suspense::Suspense>
                </HistoryProvider>
            </div>

            // Table with suspending list
            <div class="flex-1">
                <div class="bg-white rounded-lg shadow-lg border border-muted/30 overflow-hidden">
                    // Header
                    <div class="grid grid-cols-12 gap-4 bg-primary/10 px-6 py-3 font-semibold text-xs sm:text-sm text-foreground border-b border-muted/30">
                        <div class="col-span-3">{"Fecha"}</div>
                        <div class="col-span-3">{"Hora"}</div>
                        <div class="col-span-3">{"Especialidad"}</div>
                        <div class="col-span-3">{"Sala"}</div>
                    </div>

                    // Rows - with Suspense
                    <div class="divide-y divide-muted/20 max-h-[calc(84vh-14rem)] sm:max-h-[calc(90vh-14rem)] overflow-y-auto">
                        <HistoryProvider>
                            <yew::suspense::Suspense fallback={html! { <HistoryTableSkeleton /> }}>
                                <HistoryList page={*page} page_size={page_size} />
                            </yew::suspense::Suspense>
                        </HistoryProvider>
                    </div>
                </div>
            </div>

            // Pagination
            <HistoryPagination
                page={*page}
                on_next={next_page}
                on_prev={prev_page}
            />
        </div>
    }
}
