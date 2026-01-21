use yew::prelude::*;

/// Outer wrapper that conditionally mounts the calendar only when route is active
#[function_component(CalendarView)]
pub fn calendar_view() -> Html {
    let is_active = crate::router::use_is_route_active(crate::router::Route::Calendar);

    if is_active {
        html! { <CalendarViewInner /> }
    } else {
        // Return empty div to maintain layout, actual calendar won't mount
        html! { <div class="size-full" /> }
    }
}

/// Inner component that only mounts when Calendar route is active
/// This prevents data loading when the calendar is prerendered but offscreen
#[function_component(CalendarViewInner)]
fn calendar_view_inner() -> HtmlResult {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let calendar_state = use_mut_ref(|| None::<yew_full_calendar::Calendar>);
    let db_updated = crate::local_db::use_local_database_updated();
    let nav = yew_router::hooks::use_navigator();
    let is_active = crate::router::use_is_route_active(crate::router::Route::Calendar);

    // This effect only runs when the component is actually mounted (route is active)
    use_effect_with(db_updated, {
        let local_db = local_db.clone();
        let nostr_key = nostr_key.clone();
        let calendar_state = calendar_state.clone();
        move |_| {
            if is_active && let Err(e) = load_appointments(calendar_state, local_db, nostr_key) {
                web_sys::console::error_1(&format!("{e:#?}").into());
            }
        }
    });

    let on_calendar_created = {
        let calendar_state = calendar_state.clone();
        let local_db = local_db.clone();
        let nostr_key = nostr_key.clone();
        Callback::from(move |cal: yew_full_calendar::Calendar| {
            calendar_state.borrow_mut().replace(cal.clone());

            if let Err(e) =
                load_appointments(calendar_state.clone(), local_db.clone(), nostr_key.clone())
            {
                web_sys::console::error_1(&format!("{e:#?}").into());
            }
        })
    };

    let on_dates_set = {
        let calendar_state = calendar_state.clone();
        let local_db = local_db.clone();
        let nostr_key = nostr_key.clone();
        Callback::from(move |_date_set: yew_full_calendar::DateSetEvent| {
            if let Err(e) =
                load_appointments(calendar_state.clone(), local_db.clone(), nostr_key.clone())
            {
                web_sys::console::error_1(&format!("{e:#?}").into());
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

    let on_event_click = {
        let nav = nav.clone();
        Callback::from(move |event: yew_full_calendar::EventClickInfo| {
            let Some(event) = event.event() else {
                return;
            };
            let Some(nav) = nav.as_ref() else {
                return;
            };
            if let Some(id) = event.id() {
                nav.push(&crate::router::AppRoute::AppointmentDetail { id });
            }
        })
    };

    Ok(html! {
        <div class="flex flex-col gap-3 sm:gap-6 flex-1 px-4 sm:px-8 py-4 size-full">
            <div class="flex items-center justify-between gap-4">
                <paravida_components::typography::Highlight class="text-start ">
                    {"Calendario Global"}
                </paravida_components::typography::Highlight>
                <crate::shared::SyncStatus />
            </div>
            <paravida_components::Card class="!self-stretch !max-h-[calc(96vh-4rem)] !max-w-none !p-0">
            <style>
                {r"
                .fc-header-toolbar {
                    margin: 6px !important;
                }
                "}
            </style>
            <yew_full_calendar::FullCalendarComponent
                {calendar_options}
                calendar_id="admin-appointments-calendar"
                class="size-full flex-1 rounded-xl"
                {on_calendar_created}
                {on_dates_set}
                {on_event_click}
            />
            </paravida_components::Card>
        </div>
    })
}

fn load_appointments(
    calendar: std::rc::Rc<std::cell::RefCell<Option<yew_full_calendar::Calendar>>>,
    local_db: Option<UseReducerHandle<crate::local_db::IdbManager>>,
    nostr_key: Option<nostr_minions::nostro2_signer::keypair::NostrKeypair>,
) -> Result<(), web_sys::wasm_bindgen::JsValue> {
    let Some(calendar) = calendar.borrow().as_ref().cloned() else {
        return Err(web_sys::wasm_bindgen::JsValue::from_str(
            "Calendar not initialized",
        ));
    };
    let Some(local_db) = local_db.clone() else {
        return Err(web_sys::wasm_bindgen::JsValue::from_str(
            "Local DB not initialized",
        ));
    };
    let Some(nostr_key) = nostr_key else {
        return Err(web_sys::wasm_bindgen::JsValue::from_str(
            "Nostr key not initialized",
        ));
    };
    let view = calendar
        .view()
        .ok_or(web_sys::wasm_bindgen::JsValue::from_str(
            "Calendar view not initialized",
        ))?;
    let start = view
        .active_start()
        .and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(
                s.to_iso_string().as_string().unwrap_or_default().as_str(),
            )
            .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
            .ok()
        })
        .ok_or(web_sys::wasm_bindgen::JsValue::from_str(
            "Failed to get start date",
        ))?;
    let end = view
        .active_end()
        .and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(
                s.to_iso_string().as_string().unwrap_or_default().as_str(),
            )
            .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
            .ok()
        })
        .ok_or(web_sys::wasm_bindgen::JsValue::from_str(
            "Failed to get end date",
        ))?;

    yew::platform::spawn_local(async move {
        if let Err(e) = async move {
            let events = local_db
                .get_appointments_by_range_efficient(
                    start.with_timezone(&chrono::Local),
                    end.with_timezone(&chrono::Local),
                    &nostr_key,
                    Some(paravida_models::ParavidaAppointmentStatus::Booked),
                )
                .await
                .map_err(|e| {
                    web_sys::console::error_1(&format!("{e:#?}").into());
                    e
                })
                .map_err(|e| web_sys::wasm_bindgen::JsValue::from_str(&format!("{e:#?}")))?;

            let calendar_clone = calendar.clone();
            let function = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                calendar_clone.clear_events();
                for appointment in events {
                    let event = yew_full_calendar::EventBuilder::default()
                        .id(appointment.1.as_str())
                        .display(yew_full_calendar::EventDisplay::Block)
                        .text_color("#ffffff")
                        .border_color(appointment.0.room.color())
                        .background_color(appointment.0.room.color())
                        .start(yew_full_calendar::EventDate::Iso8601String(
                            appointment.0.start.to_rfc3339(),
                        ))
                        .end(yew_full_calendar::EventDate::Iso8601String(
                            appointment.0.end.to_rfc3339(),
                        ));
                    if let Err(e) = calendar_clone.add_or_replace_event(event) {
                        web_sys::console::error_1(&format!("{e:#?}").into());
                    }
                }
            });
            calendar.batch_rendering(function.into());
            Ok::<(), web_sys::wasm_bindgen::JsValue>(())
        }
        .await
        {
            web_sys::console::error_1(&format!("{e:#?}").into());
        }
    });
    Ok(())
}
