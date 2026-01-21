use yew::prelude::*;

// Re-export types from other modules
pub use crate::features::doctors::provider::{DoctorItem, DoctorsAction, DoctorsState};
use crate::features::history::provider::HistoryAction;
pub use crate::features::history::provider::HistoryData;

pub type HistoryStore = UseReducerHandle<HistoryData>;
pub type DoctorsStore = UseReducerHandle<DoctorsState>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyncStatus {
    Syncing,
    Synced,
    Disconnected,
}

pub type SyncStatusHandle = UseStateHandle<SyncStatus>;

#[function_component(NostrNotesProvider)]
pub fn nostr_notes_provider(props: &yew::html::ChildrenProps) -> HtmlResult {
    let relay_ctx = nostr_minions::use_nostr_relay_pool();
    let nostr_key = nostr_minions::use_nostr_key();
    let local_db = crate::local_db::use_local_idb_manager();
    let sub_id = use_state(String::new);

    // Initialize both stores with cached data
    let appointments_state = use_reducer_eq(|| HistoryData {
        appointments: Vec::new(),
        updated: 0,
    });

    let doctors_state = use_reducer_eq(|| DoctorsState {
        doctors: Vec::new(),
        updated: 0,
    });

    // Track sync status
    let sync_status = use_state(|| SyncStatus::Syncing);

    // Subscribe to mutual consent notes - ONCE for both types
    let relay_ctx_clone = relay_ctx.clone();
    let local_db_clone = local_db.clone();
    let sub_setter = sub_id.setter();
    use_effect_with(nostr_key.clone(), move |nostr_key| {
        let Some(nostr_key) = nostr_key else {
            return;
        };
        let Some(local_db) = local_db_clone.as_ref() else {
            return;
        };

        let local_db = local_db.clone();
        let nostr_key_str = nostr_key.public_key();
        let relay_ctx = relay_ctx_clone.clone();
        let sub_setter = sub_setter.clone();
        yew::platform::spawn_local(async move {
            // Get latest timestamp to only fetch new notes
            let latest_created_at = local_db.get_global_latest_created_at().await.ok().flatten();

            let mut filter = nostr_minions::nostro2::NostrSubscription {
                kinds: vec![crate::constants::magic_numbers::MUTUAL_CONSENT_NOTE_KIND].into(),
                since: latest_created_at.map(|created_at| created_at as u64 + 1),
                ..Default::default()
            };
            filter.add_tag("#p", &nostr_key_str);

            if let nostro2::NostrClientEvent::Subscribe(_, id, _) = relay_ctx.send(filter) {
                sub_setter.set(id);
            }
        });
    });

    // Handle incoming notes - PROCESS ONCE, branch based on type
    // let appointments_dispatch = appointments_state.dispatcher();
    // let doctors_dispatch = doctors_state.dispatcher();
    let nostr_key_clone = nostr_key.clone();
    let id = sub_id.clone();

    // We will create some buffers to ensure the IDB is not a bottleneck
    // when we have a lot of notes to process, so we rpocess them in batches

    let buffered_appointments = use_mut_ref(|| Vec::new());
    let buffered_doctors = use_mut_ref(|| Vec::new());

    let buffered_appointments_clone = buffered_appointments.clone();
    let buffered_doctors_clone = buffered_doctors.clone();

    let history_dispatch = appointments_state.dispatcher();
    let doctors_dispatch = doctors_state.dispatcher();

    let sync_status_clone = sync_status.clone();
    use_effect_with(
        (relay_ctx.last_note.clone(), local_db.clone()),
        move |(last_note, local_db_option)| {
            let Some(nostr_minions::nostro2::NostrRelayEvent::NewNote(.., note_sub_id, last_note)) =
                last_note
            else {
                return;
            };

            // Check if this note is from our subscription
            if note_sub_id.as_str() != id.as_str() {
                return;
            }

            // Check if it's the right kind
            if last_note.kind != crate::constants::magic_numbers::MUTUAL_CONSENT_NOTE_KIND {
                return;
            }

            let Some(nostr_key) = nostr_key_clone.as_ref() else {
                return;
            };

            // DECRYPT THE NOTE ONCE
            let mutual_note = mutual_consent_notes::MutualConsentNote(last_note.clone());
            let Ok(shared_document) =
                mutual_note
                    .view_shared_document(nostr_key)
                    .inspect_err(|e| {
                        web_sys::console::error_1(
                            &format!("[NostrNotesProvider] Failed to decrypt: {e:#?}").into(),
                        );
                    })
            else {
                return;
            };
            // BRANCH: Check if it's an appointment or doctor profile
            let is_appointment = shared_document
                .tags
                .0
                .iter()
                .find(|t| t.first().is_some_and(|s| s.as_str() == "fhir"))
                .and_then(|t| t.get(1));

            if is_appointment.is_some_and(|s| s.as_str() == "Appointment")
                && let Ok(appointment) =
                    paravida_models::ParavidaAppointment::from_salud_note(&shared_document)
                && let Some(appointment_id) = last_note
                    .tags
                    .0
                    .first()
                    .and_then(|t| t.get(1))
                    .map(std::string::ToString::to_string)
            {
                // Add to buffer
                buffered_appointments_clone
                    .borrow_mut()
                    .push(last_note.clone());

                if buffered_appointments_clone.borrow().len() >= 500
                    || *sync_status_clone == SyncStatus::Synced
                {
                    let Some(local_db) = local_db_option.clone() else {
                        return;
                    };
                    let appts = buffered_appointments_clone.borrow().clone();
                    yew::platform::spawn_local({
                        let nostr_key = nostr_key.clone();
                        async move {
                            if let Err(e) = local_db.save_appointment_batch(appts, &nostr_key).await
                            {
                                web_sys::console::error_1(
                                    &format!(
                                        "[NostrNotesProvider] Error saving appointments: {e:#?}"
                                    )
                                    .into(),
                                );
                            } else if *sync_status_clone == SyncStatus::Synced {
                                history_dispatch.dispatch(HistoryAction::AddAppointment(
                                    crate::features::history::provider::AppointmentItem {
                                        id: appointment_id,
                                        appointment,
                                    },
                                ));
                            }
                        }
                    });
                    buffered_appointments_clone.borrow_mut().clear();
                }
            } else if is_appointment.is_some_and(|s| s.as_str() == "Practitioner")
                && paravida_models::ParavidaPractitioner::from_salud_note(&shared_document).is_ok()
            {
                // Add to buffer
                buffered_doctors_clone.borrow_mut().push(last_note.clone());
                if let Some(local_db) = local_db_option.as_ref()
                    && (buffered_doctors_clone.borrow().len() >= 50
                        || *sync_status_clone == SyncStatus::Synced)
                {
                    let doctors = buffered_doctors_clone.borrow().clone();
                    yew::platform::spawn_local({
                        let local_db = local_db.clone();
                        let nostr_key = nostr_key.clone();
                        async move {
                            if let Err(e) = local_db.save_doctor_batch(doctors, &nostr_key).await {
                                web_sys::console::error_1(
                                    &format!("[NostrNotesProvider] Error saving doctors: {e:#?}")
                                        .into(),
                                );
                            } else if *sync_status_clone == SyncStatus::Synced {
                                doctors_dispatch.dispatch(DoctorsAction::Updated);
                            }
                        }
                    });
                    buffered_doctors_clone.borrow_mut().clear();
                }
            }
        },
    );

    // Handle EndOfSubscription (EOSE) event
    let sync_status_eose = sync_status.clone();
    let sub_id_eose = sub_id.clone();
    let local_db = local_db.clone();
    let buffered_appointments = buffered_appointments.clone();
    let buffered_doctors = buffered_doctors.clone();
    let nostr_key = nostr_key.clone();
    use_effect_with(relay_ctx.last_event.clone(), move |last_event| {
        let Some(nostr_minions::nostro2::NostrRelayEvent::EndOfSubscription(.., event_sub_id)) =
            last_event
        else {
            return;
        };

        if event_sub_id.as_str() == sub_id_eose.as_str() {
            // Spawn a task to save the buffered notes to local DB
            let Some(local_db) = local_db.clone() else {
                return;
            };
            let Some(nostr_key) = nostr_key.clone() else {
                return;
            };
            let appts = buffered_appointments.borrow().clone();
            let doctors = buffered_doctors.borrow().clone();
            yew::platform::spawn_local(async move {
                if let Err(e) = local_db.save_appointment_batch(appts, &nostr_key).await {
                    web_sys::console::error_1(
                        &format!("[NostrNotesProvider] Error saving appointments: {e:#?}").into(),
                    );
                }
                if let Err(e) = local_db.save_doctor_batch(doctors, &nostr_key).await {
                    web_sys::console::error_1(
                        &format!("[NostrNotesProvider] Error saving doctors: {e:#?}").into(),
                    );
                }
                sync_status_eose.set(SyncStatus::Synced);
            });
        }
    });

    // Handle Close event (disconnection)
    let sync_status_close = sync_status.clone();
    use_effect_with(relay_ctx.last_event.clone(), move |last_event| {
        if matches!(
            last_event,
            Some(nostr_minions::nostro2::NostrRelayEvent::Close(..))
        ) {
            web_sys::console::warn_1(&"[NostrNotesProvider] Relay disconnected".into());
            sync_status_close.set(SyncStatus::Disconnected);
        }
    });

    Ok(html! {
        <ContextProvider<HistoryStore> context={appointments_state}>
            <ContextProvider<DoctorsStore> context={doctors_state}>
                <ContextProvider<SyncStatusHandle> context={sync_status}>
                    { props.children.clone() }
                </ContextProvider<SyncStatusHandle>>
            </ContextProvider<DoctorsStore>>
        </ContextProvider<HistoryStore>>
    })
}

#[hook]
pub fn use_history_data() -> Option<HistoryStore> {
    use_context::<HistoryStore>()
}

#[hook]
pub fn use_doctors() -> Option<DoctorsStore> {
    use_context::<DoctorsStore>()
}

#[hook]
pub fn use_doctors_count() -> Option<usize> {
    let ctx = use_doctors();
    *use_memo(ctx, |ctx| ctx.as_ref().map(|ctx| ctx.doctors.len()))
}

#[hook]
pub fn use_all_doctors() -> Vec<DoctorItem> {
    let ctx = use_doctors();
    (*use_memo(ctx, |ctx| ctx.as_ref().map(|ctx| ctx.doctors.clone())))
        .clone()
        .unwrap_or_default()
}

#[hook]
pub fn use_sync_status() -> Option<SyncStatusHandle> {
    use_context::<SyncStatusHandle>()
}
