#[derive(Debug, thiserror::Error)]
pub enum LocalIdbError {
    #[error("No database context available")]
    NoDatabaseContext,
    #[error("IDB error: {0}")]
    Idb(#[from] idb::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Wasm bindgen error: {0}")]
    SerdeWasmBindgen(#[from] serde_wasm_bindgen::Error),
    #[error("No contact found")]
    NoContactFound,
    #[error("Encryption error: {0}")]
    EncryptionError(#[from] nostr_minions::nostro2_signer::nostro2_nips::Nip44Error),
    #[error("No id")]
    NoId,
}

impl From<LocalIdbError> for web_sys::wasm_bindgen::JsValue {
    fn from(value: LocalIdbError) -> Self {
        Self::from_str(&value.to_string())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppointmentMeta {
    pub app_id: String,
    pub appointment_start: i64,
    pub appointment_end: i64,
    pub status: String,
    pub created_at: i64,
}

const NOSTR_DB_NAME: &str = "nostr-paravida-admin";
const NOSTR_DB_VERSION: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalStoreName {
    Appointments,
    AppointmentMeta,
    Doctors,
    Patients,
}
impl AsRef<str> for LocalStoreName {
    fn as_ref(&self) -> &str {
        match self {
            Self::Appointments => "appointments",
            Self::AppointmentMeta => "appointment_meta",
            Self::Doctors => "doctors",
            Self::Patients => "patients",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RcIdb(std::rc::Rc<idb::Database>);
impl PartialEq for RcIdb {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::as_ptr(&self.0) == std::rc::Rc::as_ptr(&other.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdbManager {
    pub db: Option<RcIdb>,
    pub updated: u32,
}

impl IdbManager {
    #[allow(clippy::too_many_lines)]
    pub async fn new_db() -> Result<std::rc::Rc<idb::Database>, LocalIdbError> {
        let factory = idb::Factory::new()?;

        let mut open_request = factory.open(NOSTR_DB_NAME, Some(NOSTR_DB_VERSION))?;

        open_request.on_upgrade_needed(|event| {
            let database = match idb::DatabaseEvent::database(&event) {
                Ok(db) => db,
                Err(e) => {
                    web_sys::console::error_1(&format!("Error getting database: {e:#?}").into());
                    return;
                }
            };

            match database.create_object_store(
                LocalStoreName::Doctors.as_ref(),
                idb::ObjectStoreParams::new(),
            ) {
                Ok(doctors_store) => {
                    let mut params = idb::IndexParams::new();
                    params.unique(false);
                    params.multi_entry(false);

                    if doctors_store
                        .index_names()
                        .into_iter()
                        .all(|n| n != "created_at_idx")
                        && let Err(e) = doctors_store.create_index(
                            "created_at_idx",
                            idb::KeyPath::new_single("created_at"),
                            Some(params),
                        )
                    {
                        web_sys::console::error_1(
                            &format!("Error creating created_at index for doctors: {e:?}").into(),
                        );
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Error creating doctors object store: {e:#?}").into(),
                    );
                }
            }

            if let Err(e) = database.create_object_store(
                LocalStoreName::Patients.as_ref(),
                idb::ObjectStoreParams::new(),
            ) {
                web_sys::console::error_1(&format!("Error creating object store: {e:#?}").into());
            }

            match database.create_object_store(
                LocalStoreName::Appointments.as_ref(),
                idb::ObjectStoreParams::new(),
            ) {
                Ok(appointments_store) => {
                    let mut params = idb::IndexParams::new();
                    params.unique(false);
                    params.multi_entry(false);

                    if appointments_store
                        .index_names()
                        .into_iter()
                        .all(|n| n != "created_at_idx")
                        && let Err(e) = appointments_store.create_index(
                            "created_at_idx",
                            idb::KeyPath::new_single("created_at"),
                            Some(params),
                        )
                    {
                        web_sys::console::error_1(
                            &format!("Error creating created_at index: {e:?}").into(),
                        );
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Error creating object store: {e:#?}").into(),
                    );
                }
            }

            // Create appointment_meta store for efficient range queries
            let mut meta_params = idb::ObjectStoreParams::new();
            meta_params.key_path(Some(idb::KeyPath::new_single("app_id")));
            match database
                .create_object_store(LocalStoreName::AppointmentMeta.as_ref(), meta_params)
            {
                Ok(meta_store) => {
                    let mut params = idb::IndexParams::new();
                    params.unique(false);
                    params.multi_entry(false);

                    // Create index on appointment_start for efficient range queries
                    if meta_store
                        .index_names()
                        .into_iter()
                        .all(|n| n != "appointment_start_idx")
                        && let Err(e) = meta_store.create_index(
                            "appointment_start_idx",
                            idb::KeyPath::new_single("appointment_start"),
                            Some(params.clone()),
                        )
                    {
                        web_sys::console::error_1(
                            &format!("Error creating appointment_start index: {e:?}").into(),
                        );
                    }

                    // Create index on appointment_end
                    if meta_store
                        .index_names()
                        .into_iter()
                        .all(|n| n != "appointment_end_idx")
                        && let Err(e) = meta_store.create_index(
                            "appointment_end_idx",
                            idb::KeyPath::new_single("appointment_end"),
                            Some(params.clone()),
                        )
                    {
                        web_sys::console::error_1(
                            &format!("Error creating appointment_end index: {e:?}").into(),
                        );
                    }

                    // Create index on status
                    if meta_store
                        .index_names()
                        .into_iter()
                        .all(|n| n != "status_idx")
                        && let Err(e) = meta_store.create_index(
                            "status_idx",
                            idb::KeyPath::new_single("status"),
                            Some(params.clone()),
                        )
                    {
                        web_sys::console::error_1(
                            &format!("Error creating status index: {e:?}").into(),
                        );
                    }

                    // Create index on created_at
                    if meta_store
                        .index_names()
                        .into_iter()
                        .all(|n| n != "created_at_idx")
                        && let Err(e) = meta_store.create_index(
                            "created_at_idx",
                            idb::KeyPath::new_single("created_at"),
                            Some(params),
                        )
                    {
                        web_sys::console::error_1(
                            &format!("Error creating created_at index for meta: {e:?}").into(),
                        );
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Error creating appointment_meta object store: {e:#?}").into(),
                    );
                }
            }
        });
        Ok(std::rc::Rc::new(open_request.await?))
    }

    pub fn store(
        &self,
        store_name: LocalStoreName,
        mode: idb::TransactionMode,
    ) -> Result<idb::ObjectStore, LocalIdbError> {
        let db = self.db.as_ref().ok_or(LocalIdbError::NoDatabaseContext)?;
        let store =
            db.0.transaction(&[store_name.as_ref()], mode)?
                .object_store(store_name.as_ref())?;
        Ok(store)
    }

    pub async fn get_global_latest_created_at(&self) -> Result<Option<i64>, LocalIdbError> {
        Ok([
            self.get_latest_created_at().await?,
            self.get_latest_doctor_created_at().await?,
        ]
        .into_iter()
        .flatten()
        .max())
    }

    async fn get_latest_created_at(&self) -> Result<Option<i64>, LocalIdbError> {
        let store = self.store(LocalStoreName::Appointments, idb::TransactionMode::ReadOnly)?;

        let idx = store.index("created_at_idx")?;

        let Some(cursor) = idx
            .open_cursor(None, Some(idb::CursorDirection::Prev))?
            .await?
        else {
            return Ok(None);
        };

        let created_at_js = cursor.key();
        let Some(timestamp) = created_at_js?.as_f64() else {
            return Ok(None);
        };

        #[allow(clippy::cast_possible_truncation)]
        Ok(Some(timestamp.floor() as i64))
    }

    async fn get_latest_doctor_created_at(&self) -> Result<Option<i64>, LocalIdbError> {
        let store = self.store(LocalStoreName::Doctors, idb::TransactionMode::ReadOnly)?;

        let idx = store.index("created_at_idx")?;

        let Some(cursor) = idx
            .open_cursor(None, Some(idb::CursorDirection::Prev))?
            .await?
        else {
            return Ok(None);
        };

        let created_at_js = cursor.key();
        let Some(timestamp) = created_at_js?.as_f64() else {
            return Ok(None);
        };

        #[allow(clippy::cast_possible_truncation)]
        Ok(Some(timestamp as i64))
    }

    pub async fn save_appointment_batch(
        &self,
        appointments: Vec<nostro2::NostrNote>,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), LocalIdbError> {
        let db = self.db.as_ref().ok_or(LocalIdbError::NoDatabaseContext)?;

        let tx = db.0.transaction(
            &[
                LocalStoreName::Appointments.as_ref(),
                LocalStoreName::AppointmentMeta.as_ref(),
            ],
            idb::TransactionMode::ReadWrite,
        )?;

        let appointments_store = tx.object_store(LocalStoreName::Appointments.as_ref())?;
        let meta_store = tx.object_store(LocalStoreName::AppointmentMeta.as_ref())?;

        for appointment in appointments {
            let appointment_id = appointment
                .tags
                .first_parameter()
                .ok_or(LocalIdbError::NoId)?;

            // get app_id
            let mutual_note = mutual_consent_notes::MutualConsentNote(appointment.clone());
            let shared_document = mutual_note.view_shared_document(keypair).map_err(|e| {
                web_sys::console::error_1(&format!("Decryption error: {e:?}").into());
                LocalIdbError::NoContactFound // Use a generic error since we can't convert
            })?;

            let parsed_appointment = paravida_models::ParavidaAppointment::from_salud_note(
                &shared_document,
            )
            .map_err(|e| {
                web_sys::console::error_1(&format!("Failed to parse appointment: {e:?}").into());
                LocalIdbError::Serde(serde_json::from_str::<()>("invalid").unwrap_err())
            })?;

            // Create metadata
            let metadata = AppointmentMeta {
                app_id: appointment_id.clone(),
                appointment_start: parsed_appointment.start.timestamp(),
                appointment_end: parsed_appointment.end.timestamp(),
                status: parsed_appointment.status.as_ref().to_string(),
                created_at: appointment.created_at,
            };

            appointments_store
                .put(
                    &serde_wasm_bindgen::to_value(&appointment)?,
                    Some(&appointment_id.into()),
                )?
                .await?;

            meta_store
                .put(&serde_wasm_bindgen::to_value(&metadata)?, None)?
                .await?;
        }

        tx.commit()?;
        Ok(())
    }

    // pub async fn save_appointment(
    //     &self,
    //     appointment: nostro2::NostrNote,
    //     keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    // ) -> Result<(), LocalIdbError> {
    //     let db = self.db.as_ref().ok_or(LocalIdbError::NoDatabaseContext)?;

    //     let appointment_id = appointment
    //         .tags
    //         .first_parameter()
    //         .ok_or(LocalIdbError::NoId)?;

    //     // Decrypt and parse the appointment to extract metadata
    //     let mutual_note = mutual_consent_notes::MutualConsentNote(appointment.clone());
    //     let shared_document = mutual_note.view_shared_document(keypair).map_err(|e| {
    //         web_sys::console::error_1(&format!("Decryption error: {e:?}").into());
    //         LocalIdbError::NoContactFound // Use a generic error since we can't convert
    //     })?;

    //     let parsed_appointment = paravida_models::ParavidaAppointment::from_salud_note(
    //         &shared_document,
    //     )
    //     .map_err(|e| {
    //         web_sys::console::error_1(&format!("Failed to parse appointment: {e:?}").into());
    //         LocalIdbError::Serde(serde_json::from_str::<()>("invalid").unwrap_err())
    //     })?;

    //     // Create metadata
    //     let metadata = AppointmentMeta {
    //         app_id: appointment_id.clone(),
    //         appointment_start: parsed_appointment.start.timestamp(),
    //         appointment_end: parsed_appointment.end.timestamp(),
    //         status: parsed_appointment.status.as_ref().to_string(),
    //         created_at: appointment.created_at,
    //     };

    //     // Use a single transaction for both stores
    //     let transaction = db.0.transaction(
    //         &[
    //             LocalStoreName::Appointments.as_ref(),
    //             LocalStoreName::AppointmentMeta.as_ref(),
    //         ],
    //         idb::TransactionMode::ReadWrite,
    //     )?;

    //     // Save appointment note
    //     let appointments_store = transaction.object_store(LocalStoreName::Appointments.as_ref())?;
    //     let request = appointments_store.put(
    //         &serde_wasm_bindgen::to_value(&appointment)?,
    //         Some(&web_sys::wasm_bindgen::JsValue::from_str(&appointment_id)),
    //     );
    //     request?.await?;

    //     // Save metadata
    //     let meta_store = transaction.object_store(LocalStoreName::AppointmentMeta.as_ref())?;
    //     let meta_request = meta_store.put(
    //         &serde_wasm_bindgen::to_value(&metadata)?,
    //         None, // app_id is the keyPath
    //     );
    //     meta_request?.await?;

    //     Ok(())
    // }

    pub async fn get_appointment(&self, id: &str) -> Result<nostro2::NostrNote, LocalIdbError> {
        let store = self.store(LocalStoreName::Appointments, idb::TransactionMode::ReadOnly)?;
        let request = store
            .get(web_sys::wasm_bindgen::JsValue::from_str(id))?
            .await?;
        let Some(request) = request else {
            return Err(LocalIdbError::NoId);
        };
        Ok(serde_wasm_bindgen::from_value(request)?)
    }

    pub async fn get_appointments_paginated(
        &self,
        after_created_at: Option<i64>,
        page_size: usize,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<
        (
            Vec<(paravida_models::ParavidaAppointment, String)>,
            Option<i64>,
        ),
        LocalIdbError,
    > {
        let store = self.store(LocalStoreName::Appointments, idb::TransactionMode::ReadOnly)?;

        // Use created_at index to iterate in reverse (newest first)
        let idx = store.index("created_at_idx")?;

        // Create key range to start from after_created_at if provided
        let query = if let Some(timestamp) = after_created_at {
            // We want all records with created_at < timestamp (going backwards)
            let key_range = idb::KeyRange::upper_bound(
                #[allow(clippy::cast_precision_loss)]
                &web_sys::wasm_bindgen::JsValue::from_f64(timestamp as f64),
                Some(true),
            )?;
            Some(idb::Query::Key(key_range.into()))
        } else {
            None
        };

        let Some(mut cursor) = idx
            .open_cursor(query, Some(idb::CursorDirection::Prev))?
            .await?
        else {
            return Ok((Vec::new(), None));
        };

        let mut results = Vec::new();
        let mut last_created_at: Option<i64> = None;

        loop {
            if let Ok(value) = cursor.value()
                && let Ok(note) = serde_wasm_bindgen::from_value::<nostro2::NostrNote>(value)
            {
                // Track the created_at for cursor positioning
                let created_at = note.created_at;

                let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                let Some(app_id) = mutual_note
                    .0
                    .tags
                    .0
                    .first()
                    .and_then(|t| t.get(1))
                    .map(std::string::ToString::to_string)
                else {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                };

                let Ok(shared_document) = mutual_note.view_shared_document(keypair) else {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                };

                let Ok(appointment) =
                    paravida_models::ParavidaAppointment::from_salud_note(&shared_document)
                else {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                };

                // Only include booked appointments
                if appointment.status != paravida_models::ParavidaAppointmentStatus::Booked {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                }

                // Collect results for this page
                results.push((appointment, app_id));
                last_created_at = Some(created_at);

                if results.len() >= page_size {
                    break;
                }
            }

            if let Some(next_cursor) = cursor.next(None)?.await? {
                cursor = next_cursor;
            } else {
                break;
            }
        }

        Ok((results, last_created_at))
    }

    #[allow(clippy::too_many_lines)]
    /// Efficient range query using metadata store - O(matches) instead of O(N)
    pub async fn get_appointments_by_range_efficient(
        &self,
        start: chrono::DateTime<chrono::Local>,
        end: chrono::DateTime<chrono::Local>,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
        status_filter: Option<paravida_models::ParavidaAppointmentStatus>,
    ) -> Result<Vec<(paravida_models::ParavidaAppointment, String)>, LocalIdbError> {
        let start_ts = start.timestamp();
        let end_ts = end.timestamp();

        let db = self.db.as_ref().ok_or(LocalIdbError::NoDatabaseContext)?;
        let transaction = db.0.transaction(
            &[
                LocalStoreName::AppointmentMeta.as_ref(),
                LocalStoreName::Appointments.as_ref(),
            ],
            idb::TransactionMode::ReadOnly,
        )?;

        // Query metadata store using appointment_start index
        let meta_store = transaction.object_store(LocalStoreName::AppointmentMeta.as_ref())?;
        let idx = meta_store.index("appointment_start_idx")?;

        // Create key range: appointment_start < end_ts
        let key_range = idb::KeyRange::upper_bound(
            #[allow(clippy::cast_precision_loss)]
            &web_sys::wasm_bindgen::JsValue::from_f64(end_ts as f64),
            Some(true),
        )?;
        let query = idb::Query::Key(key_range.into());

        let Some(mut cursor) = idx
            .open_cursor(Some(query), Some(idb::CursorDirection::Next))?
            .await?
        else {
            return Ok(Vec::new());
        };

        // Collect matching app_ids from metadata
        let mut matching_ids = Vec::new();
        loop {
            if let Ok(value) = cursor.value()
                && let Ok(meta) = serde_wasm_bindgen::from_value::<AppointmentMeta>(value)
            {
                // Check if appointment_end > start_ts (overlaps with query range)
                if meta.appointment_end <= start_ts {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                }

                // Check status filter if provided
                if let Some(ref status) = status_filter
                    && meta.status != status.as_ref()
                {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                }

                matching_ids.push(meta.app_id);
            }

            if let Some(next_cursor) = cursor.next(None)?.await? {
                cursor = next_cursor;
            } else {
                break;
            }
        }

        // Fetch and decrypt only the matching appointments
        matching_ids.sort();

        // If empty, early return
        if matching_ids.is_empty() {
            return Ok(Vec::new());
        }

        let app_store = transaction.object_store(LocalStoreName::Appointments.as_ref())?;

        let raw_results = {
            let mut results = Vec::new();
            for id in matching_ids {
                let Ok(id_req) = app_store.get(web_sys::wasm_bindgen::JsValue::from_str(&id))
                else {
                    continue;
                };
                let Ok(Some(note_js)) = id_req.await else {
                    continue;
                };
                let Ok(note) = serde_wasm_bindgen::from_value::<nostro2::NostrNote>(note_js) else {
                    continue;
                };
                let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                let Ok(shared_document) = mutual_note.view_shared_document(keypair).inspect_err(|e| {
                        web_sys::console::error_1(
                            &format!(
                                "[get_appointments_by_range_efficient] Error decrypting note: {e:#?}"
                            )
                            .into(),
                        );
                    }) else {
                        continue;
                    };
                let Ok(appointment) =
                            paravida_models::ParavidaAppointment::from_salud_note(&shared_document)
                                .inspect_err(|e| {
                                    web_sys::console::error_1(
                                        &format!(
                                            "[get_appointments_by_range_efficient] Error parsing appointment: {e:#?}"
                                        )
                                        .into(),
                                    );
                                }) else {
                                    continue;
                                };
                let Some(app_id) = mutual_note.0.tags.first_parameter() else {
                    continue;
                };
                results.push((appointment, app_id));
            }
            results
        };

        transaction.commit()?;
        Ok(raw_results)
    }

    pub async fn save_doctor_batch(
        &self,
        doctors: Vec<nostro2::NostrNote>,
        keypair: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<(), LocalIdbError> {
        let db = self.db.as_ref().ok_or(LocalIdbError::NoDatabaseContext)?;

        let tx = db.0.transaction(
            &[LocalStoreName::Doctors.as_ref()],
            idb::TransactionMode::ReadWrite,
        )?;

        let doctors_store = tx.object_store(LocalStoreName::Doctors.as_ref())?;

        for doctor in doctors {
            let Some(doctor_pubkey) = doctor
                .tags
                .0
                .iter()
                .filter(|t| t.first().is_some_and(|s| s.as_str() == "p"))
                .filter_map(|t| t.get(1).as_ref().map(|s| (*s).to_string()))
                .find(|t| {
                    t.as_str() != keypair.public_key().as_str()
                        && t.as_str() != crate::PARAVIDA_PUBKEY
                })
            else {
                continue;
            };

            let existing_val = doctors_store
                .get(web_sys::wasm_bindgen::JsValue::from_str(&doctor_pubkey))?
                .await?;

            if let Some(js_value) = existing_val
                && let Ok(existing_note) =
                    serde_wasm_bindgen::from_value::<nostro2::NostrNote>(js_value)
                && existing_note.created_at >= doctor.created_at
            {
                return Ok(());
            }

            // Newer → overwrite
            doctors_store
                .put(
                    &serde_wasm_bindgen::to_value(&doctor)?,
                    Some(&doctor_pubkey.into()),
                )?
                .await?;
        }

        tx.commit()?;
        Ok(())
    }

    pub async fn get_doctor(
        &self,
        doctor_pubkey: &str,
    ) -> Result<Option<nostro2::NostrNote>, LocalIdbError> {
        let store = self.store(LocalStoreName::Doctors, idb::TransactionMode::ReadOnly)?;
        let request = store
            .get(web_sys::wasm_bindgen::JsValue::from_str(doctor_pubkey))?
            .await?;
        Ok(request.and_then(|value| serde_wasm_bindgen::from_value(value).ok()))
    }

    /// Get doctors paginated with optional search filter
    /// Filters by name or specialty (case-insensitive)
    /// Returns (results, `last_created_at_cursor`, `total_matching_count`)
    pub async fn get_doctors_paginated_filtered(
        &self,
        after_created_at: Option<i64>,
        page_size: usize,
        search_query: Option<String>,
        nostr_key: &nostr_minions::nostro2_signer::keypair::NostrKeypair,
    ) -> Result<
        (
            Vec<(String, nostro2::NostrNote)>,
            Option<i64>,
            usize, // total matching count
        ),
        LocalIdbError,
    > {
        let store = self.store(LocalStoreName::Doctors, idb::TransactionMode::ReadOnly)?;
        let idx = store.index("created_at_idx")?;

        let query = if let Some(timestamp) = after_created_at {
            let key_range = idb::KeyRange::upper_bound(
                #[allow(clippy::cast_precision_loss)]
                &web_sys::wasm_bindgen::JsValue::from_f64(timestamp as f64),
                Some(true),
            )?;
            Some(idb::Query::Key(key_range.into()))
        } else {
            None
        };

        let Some(mut cursor) = idx
            .open_cursor(query, Some(idb::CursorDirection::Prev))?
            .await?
        else {
            return Ok((Vec::new(), None, 0));
        };

        let mut results = Vec::new();
        let mut last_created_at: Option<i64> = None;
        let mut total_matching = 0;

        // Normalize search query once
        let search_lower = search_query.as_ref().map(|q| q.trim().to_lowercase());

        loop {
            if let Ok(value) = cursor.value()
                && let Ok(note) = serde_wasm_bindgen::from_value::<nostro2::NostrNote>(value)
            {
                let created_at = note.created_at;
                let pubkey_js = cursor.primary_key()?;
                let Some(pubkey) = pubkey_js.as_string() else {
                    if let Some(next_cursor) = cursor.next(None)?.await? {
                        cursor = next_cursor;
                        continue;
                    }
                    break;
                };

                // Apply search filter if provided
                let matches = if let Some(ref query_lower) = search_lower {
                    // Decrypt and parse the doctor to check name/specialty
                    let mutual_note = mutual_consent_notes::MutualConsentNote(note.clone());
                    if let Ok(shared_doc) = mutual_note.view_shared_document(nostr_key) {
                        if let Ok(practitioner) =
                            paravida_models::ParavidaPractitioner::from_salud_note(&shared_doc)
                        {
                            let name_lower = practitioner.name.text().to_lowercase();
                            let specialty_lower = practitioner
                                .specialty
                                .first()
                                .map(|s| s.display_text().to_lowercase())
                                .unwrap_or_default();

                            name_lower.contains(query_lower)
                                || specialty_lower.contains(query_lower)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    true // No filter, match all
                };

                if matches {
                    total_matching += 1;

                    // Only collect if we haven't filled the page yet
                    if results.len() < page_size {
                        results.push((pubkey, note));
                        last_created_at = Some(created_at);
                    }
                }
            }

            if let Some(next_cursor) = cursor.next(None)?.await? {
                cursor = next_cursor;
            } else {
                break;
            }
        }

        Ok((results, last_created_at, total_matching))
    }
}

use yew::prelude::*;
pub enum IdbManagerAction {}

impl Reducible for IdbManager {
    type Action = IdbManagerAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {}
    }
}

pub type IdbStore = UseReducerHandle<IdbManager>;

#[function_component(LocalIdbManagerProvider)]
pub fn key_handler(props: &yew::html::ChildrenProps) -> HtmlResult {
    let db = yew::suspense::use_future_with((), |_| async move {
        match IdbManager::new_db().await {
            Ok(db) => Some(db),
            Err(e) => {
                web_sys::console::error_1(
                    &format!("[LocalIdbManagerProvider] DB initialization failed: {e:#?}").into(),
                );
                None
            }
        }
    })?
    .as_ref()
    .cloned();

    let ctx = use_reducer_eq(|| IdbManager {
        db: db.map(RcIdb),
        updated: 0,
    });
    Ok(html! {
        <ContextProvider<IdbStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<IdbStore>>
    })
}

#[hook]
pub fn use_local_idb_manager() -> Option<IdbStore> {
    use_context::<IdbStore>()
}

#[hook]
pub fn use_local_database() -> Option<std::rc::Rc<idb::Database>> {
    let ctx = use_context::<IdbStore>().expect("No IDB store");
    (*use_memo(ctx.db.clone(), |db| db.clone().map(|db| db.0))).clone()
}

#[hook]
pub fn use_local_database_updated() -> bool {
    let ctx = use_context::<IdbStore>().expect("No IDB store");
    *use_memo(ctx.updated, |updated| *updated > 0)
}
