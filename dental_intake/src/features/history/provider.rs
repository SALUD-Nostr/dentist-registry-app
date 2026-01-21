use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppointmentItem {
    pub id: String,
    pub appointment: paravida_models::ParavidaAppointment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryData {
    pub appointments: Vec<AppointmentItem>,
    pub updated: u32,
}

#[allow(dead_code)]
pub enum HistoryAction {
    AddAppointment(AppointmentItem),
    SetAppointments(Vec<AppointmentItem>),
    Updated,
}

impl Reducible for HistoryData {
    type Action = HistoryAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut state = (*self).clone();
        match action {
            HistoryAction::AddAppointment(appointment) => {
                // Remove existing appointment with same ID, then add new one
                state.appointments.retain(|a| a.id != appointment.id);
                state.appointments.push(appointment);
                state.updated += 1;
            }
            HistoryAction::SetAppointments(appointments) => {
                state.appointments = appointments;
                state.updated += 1;
            }
            HistoryAction::Updated => {
                state.updated += 1;
            }
        }
        std::rc::Rc::new(state)
    }
}

pub type HistoryStore = UseReducerHandle<HistoryData>;

#[function_component(HistoryDataProvider)]
pub fn history_data_provider(props: &yew::html::ChildrenProps) -> HtmlResult {
    let relay_ctx = nostr_minions::use_nostr_relay_pool();
    let nostr_key = nostr_minions::use_nostr_key();
    let local_db = crate::local_db::use_local_idb_manager();
    let sub_id = use_state(String::new);

    // Load appointments from local DB - depends on local_db and nostr_key so it re-runs when they're ready
    // let cached_appointments =
    //     yew::suspense::use_future_with((local_db.clone(), nostr_key.clone()), |deps| {
    //         let (local_db, nostr_key) = deps.as_ref().clone();
    //         async move {
    //             let Some(local_db) = local_db.as_ref() else {
    //                 web_sys::console::log_1(&"[HistoryDataProvider] No local_db available".into());
    //                 return Vec::new();
    //             };

    //             let Some(nostr_key) = nostr_key.as_ref() else {
    //                 web_sys::console::log_1(&"[HistoryDataProvider] No nostr_key available".into());
    //                 return Vec::new();
    //             };

    //             // Get appointments from the last month (or adjust time range as needed)
    //             let end = chrono::Local::now();
    //             let start = end - chrono::Duration::days(30);

    //             match local_db
    //                 .get_appointments_by_range_efficient(
    //                     start,
    //                     end,
    //                     nostr_key,
    //                     Some(paravida_models::ParavidaAppointmentStatus::Booked),
    //                 )
    //                 .await
    //             {
    //                 Ok(appointments) => {
    //                     let items: Vec<AppointmentItem> = appointments
    //                         .into_iter()
    //                         .map(|(appointment, id)| AppointmentItem { id, appointment })
    //                         .collect();

    //                     items
    //                 }
    //                 Err(e) => {
    //                     web_sys::console::error_1(
    //                         &format!("[HistoryDataProvider] Error loading appointments: {e:#?}")
    //                             .into(),
    //                     );
    //                     Vec::new()
    //                 }
    //             }
    //         }
    //     })?;

    let state = use_reducer_eq(|| HistoryData {
        appointments: Vec::new(),
        // appointments: cached_appointments.clone(),
        updated: 0,
    });

    // Subscribe to appointment events
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
            // Get latest appointment timestamp to only fetch new ones
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

    // Handle incoming appointment notes
    let dispatch = state.dispatcher();
    let nostr_key_clone = nostr_key.clone();
    let id = sub_id.clone();
    use_effect_with(
        (relay_ctx.last_note.clone(), local_db.clone()),
        move |(last_note, _local_db_option)| {
            let Some(nostr_minions::nostro2::NostrRelayEvent::NewNote(.., note_sub_id, last_note)) =
                last_note
            else {
                return;
            };
            if note_sub_id.as_str() != id.as_str() {
                return;
            }

            let Some(nostr_key) = nostr_key_clone.as_ref() else {
                return;
            };

            // Decrypt the mutual consent note
            let mutual_note = mutual_consent_notes::MutualConsentNote(last_note.clone());
            let Ok(shared_document) =
                mutual_note
                    .view_shared_document(nostr_key)
                    .inspect_err(|e| {
                        web_sys::console::error_1(&format!("Failed to decrypt: {e:#?}").into());
                    })
            else {
                return;
            };

            // Check if it's an appointment (fhir=Appointment tag)
            let is_appointment = shared_document
                .tags
                .0
                .iter()
                .filter(|t| t.first().is_some_and(|s| s.as_str() == "fhir"))
                .filter_map(|t| t.get(1))
                .any(|s| s.as_str() == "Appointment");

            if !is_appointment {
                return;
            }

            // Parse the appointment
            let Ok(appointment) =
                paravida_models::ParavidaAppointment::from_salud_note(&shared_document)
            else {
                web_sys::console::error_1(
                    &"[HistoryDataProvider] Failed to parse appointment".into(),
                );
                return;
            };

            let Some(appointment_id) = last_note
                .tags
                .0
                .first()
                .and_then(|t| t.get(1))
                .map(std::string::ToString::to_string)
            else {
                web_sys::console::error_1(&"[HistoryDataProvider] No appointment ID found".into());
                return;
            };

            // Update state
            dispatch.dispatch(HistoryAction::AddAppointment(AppointmentItem {
                id: appointment_id,
                appointment,
            }));
        },
    );

    Ok(html! {
        <ContextProvider<HistoryStore> context={state}>
            { props.children.clone() }
        </ContextProvider<HistoryStore>>
    })
}

#[hook]
pub fn use_history_data() -> Option<HistoryStore> {
    use_context::<HistoryStore>()
}
