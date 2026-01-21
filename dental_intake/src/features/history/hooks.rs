use std::collections::HashMap;
use yew::prelude::*;

// Context to store cursor positions for each page
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaginationCursors {
    // Maps page number to the created_at timestamp where that page should start
    pub cursors: HashMap<usize, Option<i64>>,
}

impl Default for PaginationCursors {
    fn default() -> Self {
        let mut cursors = HashMap::new();
        cursors.insert(0, None); // Page 0 starts from the beginning (no cursor)
        Self { cursors }
    }
}

#[hook]
pub fn use_pagination_cursors() -> UseStateHandle<PaginationCursors> {
    use_state(PaginationCursors::default)
}

#[hook]
pub fn use_appointment_list(
    page: usize,
    page_size: usize,
) -> std::result::Result<
    yew::suspense::UseFutureHandle<Vec<(paravida_models::ParavidaAppointment, String)>>,
    yew::suspense::Suspension,
> {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let db_updated = crate::local_db::use_local_database_updated();
    let sync_status = crate::features::nostr_notes::use_sync_status();
    let cursors = use_context::<UseStateHandle<PaginationCursors>>();
    let is_active = crate::router::use_is_route_active(crate::router::Route::History);

    yew::suspense::use_future_with(
        (db_updated, sync_status.clone(), page, page_size, is_active),
        |deps| {
            let local_db = local_db.clone();
            let nostr_key = nostr_key.clone();
            let cursors = cursors.clone();
            async move {
                let (_, _sync_status, page, page_size, is_active) = deps.as_ref();
                if !is_active {
                    return Vec::new();
                }
                let nostr_key = match nostr_key.as_ref() {
                    Some(key) => key,
                    None => return Vec::new(),
                };
                let local_db = match local_db.clone() {
                    Some(db) => db,
                    None => return Vec::new(),
                };

                // Get the cursor position for this page
                let after_created_at = if let Some(cursors_state) = cursors.as_ref() {
                    cursors_state.cursors.get(page).copied().flatten()
                } else {
                    None
                };

                // Use paginated query with cursor
                let (appointments, last_created_at) = local_db
                    .get_appointments_paginated(after_created_at, *page_size, nostr_key)
                    .await
                    .inspect_err(|e| {
                        web_sys::console::error_1(
                            &format!("[use_appointment_list] Error: {e:#?}").into(),
                        );
                    })
                    .unwrap_or_default();

                // Store the cursor for the next page
                if let Some(cursors_state) = cursors.as_ref()
                    && let Some(last_ts) = last_created_at
                {
                    let mut new_cursors = (**cursors_state).clone();
                    new_cursors.cursors.insert(*page + 1, Some(last_ts));
                    cursors_state.set(new_cursors);
                }

                appointments
            }
        },
    )
}

#[hook]
pub fn use_appointment() -> std::result::Result<
    yew::suspense::UseFutureHandle<
        std::option::Option<(String, paravida_models::ParavidaAppointment)>,
    >,
    yew::suspense::Suspension,
> {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let route = yew_router::hooks::use_route::<crate::router::AppRoute>();
    let db_updated = crate::local_db::use_local_database_updated();
    let sync_status = crate::features::nostr_notes::use_sync_status();

    yew::suspense::use_future_with((db_updated, sync_status.clone(), route.clone()), |deps| {
        let local_db = local_db.clone();
        let nostr_key = nostr_key.clone();
        async move {
            let (_, _sync_status, route) = deps.as_ref();
            let Some(crate::router::AppRoute::AppointmentDetail { id }) = route.as_ref() else {
                return None;
            };
            let nostr_key = nostr_key.as_ref()?;
            let local_db = local_db.clone()?;

            let note = local_db
                .get_appointment(id.as_str())
                .await
                .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                .ok()
                .and_then(|note| {
                    let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                    mutual_note
                        .view_shared_document(nostr_key)
                        .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                        .ok()
                })
                .and_then(|shared_note| {
                    paravida_models::ParavidaAppointment::from_salud_note(&shared_note)
                        .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                        .ok()
                })?;

            Some((id.to_string(), note))
        }
    })
}
