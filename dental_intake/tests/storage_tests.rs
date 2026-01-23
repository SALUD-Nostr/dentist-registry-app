//! Storage layer E2E tests
//!
//! These tests run in a real browser environment and test IndexedDB operations.
//! Run with: wasm-pack test --headless --chrome

use chrono::{NaiveDate, Utc};
use salud_types::{
    Address, AdministrativeGender, ClinicalImpressionBuilder, ClinicalImpressionStatus,
    CodeableConcept, ContactPoint, ContactPointSystem, ContactPointUse, EncounterBuilder,
    EncounterClass, EncounterStatus, HumanName, PatientBuilder, Period, Reference,
};
use uuid::Uuid;
use wasm_bindgen_test::*;

// Configure tests to run in browser
wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to create a test patient
fn create_test_patient() -> salud_types::Patient {
    PatientBuilder::default()
        .id(Uuid::new_v4().to_string())
        .active(true)
        .name(vec![HumanName {
            text: Some("Juan Pérez".to_string()),
            family: Some("Pérez".to_string()),
            given: Some(vec!["Juan".to_string()]),
        }])
        .gender(AdministrativeGender::Male)
        .birth_date(NaiveDate::from_ymd_opt(1980, 5, 15).unwrap())
        .telecom(vec![
            ContactPoint {
                system: ContactPointSystem::Phone,
                value: "+34612345678".to_string(),
                use_: Some(ContactPointUse::Mobile),
            },
            ContactPoint {
                system: ContactPointSystem::Email,
                value: "juan.perez@example.com".to_string(),
                use_: Some(ContactPointUse::Home),
            },
        ])
        .address(vec![Address {
            text: None,
            line: Some(vec!["Calle Mayor 123".to_string()]),
            city: Some("Madrid".to_string()),
            state: Some("Madrid".to_string()),
            postal_code: Some("28001".to_string()),
            country: Some("España".to_string()),
        }])
        .build()
        .unwrap()
}

/// Helper function to create a test encounter
fn create_test_encounter(patient_id: &str) -> salud_types::Encounter {
    let start_time = chrono::Utc::now();

    EncounterBuilder::default()
        .id(Uuid::new_v4().to_string())
        .status(EncounterStatus::Planned)
        .class(EncounterClass::Ambulatory)
        .subject(Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: Some("Test Patient".to_string()),
            type_: None,
        })
        .period(Period {
            start: Some(start_time),
            end: None,
        })
        .reason_code(vec![CodeableConcept {
            text: "Revisión dental general".to_string(),
            coding: None,
        }])
        .build()
        .unwrap()
}

/// Helper function to create a test clinical impression
fn create_test_clinical_impression(
    patient_id: &str,
    encounter_id: &str,
) -> salud_types::ClinicalImpression {
    ClinicalImpressionBuilder::default()
        .id(Uuid::new_v4().to_string())
        .status(ClinicalImpressionStatus::Completed)
        .subject(Reference {
            reference: Some(format!("Patient/{}", patient_id)),
            display: Some("Test Patient".to_string()),
            type_: None,
        })
        .encounter(Reference {
            reference: Some(format!("Encounter/{}", encounter_id)),
            display: None,
            type_: None,
        })
        .effective_date_time(Utc::now())
        .summary("Revisión dental completada sin hallazgos significativos".to_string())
        .build()
        .unwrap()
}

#[wasm_bindgen_test]
async fn test_patient_store_crud_operations() {
    // Import the storage module
    use dental_intake::storage::PatientStore;

    // Create a patient store
    let store = PatientStore::new()
        .await
        .expect("Failed to create patient store");

    // Create a test patient
    let patient = create_test_patient();
    let patient_id = patient.id.clone().expect("Patient should have ID");

    // Test CREATE
    store.save(&patient).await.expect("Failed to save patient");

    // Test READ
    let retrieved = store
        .get(&patient_id)
        .await
        .expect("Failed to get patient")
        .expect("Patient not found");

    assert_eq!(retrieved.id, patient.id);
    assert_eq!(
        retrieved
            .name
            .as_ref()
            .and_then(|n| n.first())
            .and_then(|n| n.family.as_ref()),
        Some(&"Pérez".to_string())
    );
    assert_eq!(retrieved.gender, Some(AdministrativeGender::Male));

    // Test UPDATE
    let mut updated_patient = retrieved;
    if let Some(ref mut names) = updated_patient.name
        && let Some(name) = names.get_mut(0)
    {
        name.given = Some(vec!["Juan Carlos".to_string()]);
    }

    store
        .save(&updated_patient)
        .await
        .expect("Failed to update patient");

    let retrieved_updated = store
        .get(&patient_id)
        .await
        .expect("Failed to get updated patient")
        .expect("Updated patient not found");

    assert_eq!(
        retrieved_updated
            .name
            .as_ref()
            .and_then(|n| n.first())
            .and_then(|n| n.given.as_ref()),
        Some(&vec!["Juan Carlos".to_string()])
    );

    // Test LIST
    let all_patients = store.get_all().await.expect("Failed to list patients");

    assert!(!all_patients.is_empty());
    assert!(all_patients.iter().any(|p| p.id == patient.id));

    // Test DELETE
    store
        .delete(&patient_id)
        .await
        .expect("Failed to delete patient");

    let deleted = store
        .get(&patient_id)
        .await
        .expect("Failed to check deleted patient");

    assert!(deleted.is_none());
}

#[wasm_bindgen_test]
async fn test_encounter_store_crud_operations() {
    use dental_intake::storage::{EncounterStore, PatientStore};

    // Create patient store which initializes the database
    let patient_store = PatientStore::new()
        .await
        .expect("Failed to create patient store");

    // Create encounter store using the same database
    let encounter_store = EncounterStore::from_db(patient_store.db.clone());

    // Create and save a test patient
    let patient = create_test_patient();
    let patient_id = patient.id.clone().expect("Patient should have ID");
    patient_store
        .save(&patient)
        .await
        .expect("Failed to save patient");

    // Create a test encounter
    let encounter = create_test_encounter(&patient_id);
    let encounter_id = encounter.id.clone().expect("Encounter should have ID");

    // Test CREATE
    encounter_store
        .save(&encounter)
        .await
        .expect("Failed to save encounter");

    // Test READ
    let retrieved = encounter_store
        .get(&encounter_id)
        .await
        .expect("Failed to get encounter")
        .expect("Encounter not found");

    assert_eq!(retrieved.id, encounter.id);
    assert_eq!(retrieved.status, EncounterStatus::Planned);

    // Test UPDATE status
    let mut updated_encounter = retrieved;
    updated_encounter.status = EncounterStatus::Finished;

    encounter_store
        .save(&updated_encounter)
        .await
        .expect("Failed to update encounter");

    let retrieved_updated = encounter_store
        .get(&encounter_id)
        .await
        .expect("Failed to get updated encounter")
        .expect("Updated encounter not found");

    assert_eq!(retrieved_updated.status, EncounterStatus::Finished);

    // Test get by patient
    let patient_encounters = encounter_store
        .get_by_patient(&patient_id)
        .await
        .expect("Failed to get encounters by patient");

    assert!(!patient_encounters.is_empty());
    assert!(patient_encounters.iter().any(|e| e.id == encounter.id));

    // Test DELETE
    encounter_store
        .delete(&encounter_id)
        .await
        .expect("Failed to delete encounter");

    let deleted = encounter_store
        .get(&encounter_id)
        .await
        .expect("Failed to check deleted encounter");

    assert!(deleted.is_none());
}

#[wasm_bindgen_test]
async fn test_clinical_impression_store_crud_operations() {
    use dental_intake::storage::{ClinicalImpressionStore, EncounterStore, PatientStore};

    // Create patient store which initializes the database
    let patient_store = PatientStore::new()
        .await
        .expect("Failed to create patient store");

    // Create other stores using the same database
    let encounter_store = EncounterStore::from_db(patient_store.db.clone());
    let impression_store = ClinicalImpressionStore::from_db(patient_store.db.clone());

    // Create and save test patient and encounter
    let patient = create_test_patient();
    let patient_id = patient.id.clone().expect("Patient should have ID");
    patient_store
        .save(&patient)
        .await
        .expect("Failed to save patient");

    let encounter = create_test_encounter(&patient_id);
    let encounter_id = encounter.id.clone().expect("Encounter should have ID");
    encounter_store
        .save(&encounter)
        .await
        .expect("Failed to save encounter");

    // Create a test clinical impression
    let impression = create_test_clinical_impression(&patient_id, &encounter_id);
    let impression_id = impression.id.clone().expect("Impression should have ID");

    // Test CREATE
    impression_store
        .save(&impression)
        .await
        .expect("Failed to save clinical impression");

    // Test READ
    let retrieved = impression_store
        .get(&impression_id)
        .await
        .expect("Failed to get clinical impression")
        .expect("Clinical impression not found");

    assert_eq!(retrieved.id, impression.id);
    assert_eq!(retrieved.status, ClinicalImpressionStatus::Completed);
    assert_eq!(
        retrieved.summary.as_deref(),
        Some("Revisión dental completada sin hallazgos significativos")
    );

    // Test get by encounter
    let encounter_impressions = impression_store
        .get_by_encounter(&encounter_id)
        .await
        .expect("Failed to get impressions by encounter");

    assert!(!encounter_impressions.is_empty());
    assert!(encounter_impressions.iter().any(|i| i.id == impression.id));

    // Test DELETE
    impression_store
        .delete(&impression_id)
        .await
        .expect("Failed to delete clinical impression");

    let deleted = impression_store
        .get(&impression_id)
        .await
        .expect("Failed to check deleted impression");

    assert!(deleted.is_none());
}

#[wasm_bindgen_test]
async fn test_data_persistence_across_store_instances() {
    use dental_intake::storage::PatientStore;

    // Create patient in first store instance
    let patient = create_test_patient();
    let patient_id = patient.id.clone().expect("Patient should have ID");

    {
        let store1 = PatientStore::new()
            .await
            .expect("Failed to create first store");

        store1.save(&patient).await.expect("Failed to save patient");
    }

    // Retrieve patient in second store instance (simulates app reload)
    {
        let store2 = PatientStore::new()
            .await
            .expect("Failed to create second store");

        let retrieved = store2
            .get(&patient_id)
            .await
            .expect("Failed to get patient from second store")
            .expect("Patient not found in second store");

        assert_eq!(retrieved.id, patient.id);
        assert_eq!(
            retrieved
                .name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.family.as_ref()),
            Some(&"Pérez".to_string())
        );
    }
}

#[wasm_bindgen_test]
async fn test_search_patients_by_name() {
    use dental_intake::storage::PatientStore;

    let store = PatientStore::new()
        .await
        .expect("Failed to create patient store");

    // Create multiple test patients
    let patient1 = {
        let mut p = create_test_patient();
        p.id = Some(Uuid::new_v4().to_string());
        if let Some(ref mut names) = p.name
            && let Some(name) = names.get_mut(0)
        {
            name.family = Some("García".to_string());
            name.given = Some(vec!["María".to_string()]);
        }
        p
    };

    let patient2 = {
        let mut p = create_test_patient();
        p.id = Some(Uuid::new_v4().to_string());
        if let Some(ref mut names) = p.name
            && let Some(name) = names.get_mut(0)
        {
            name.family = Some("Martínez".to_string());
            name.given = Some(vec!["Carlos".to_string()]);
        }
        p
    };

    let patient3 = {
        let mut p = create_test_patient();
        p.id = Some(Uuid::new_v4().to_string());
        if let Some(ref mut names) = p.name
            && let Some(name) = names.get_mut(0)
        {
            name.family = Some("García".to_string());
            name.given = Some(vec!["Ana".to_string()]);
        }
        p
    };

    // Save all patients
    store
        .save(&patient1)
        .await
        .expect("Failed to save patient1");
    store
        .save(&patient2)
        .await
        .expect("Failed to save patient2");
    store
        .save(&patient3)
        .await
        .expect("Failed to save patient3");

    // Search by family name
    let all_patients = store.get_all().await.expect("Failed to list patients");
    let garcia_patients: Vec<_> = all_patients
        .iter()
        .filter(|p| {
            p.name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.family.as_ref())
                .map(|f| f.contains("García"))
                .unwrap_or(false)
        })
        .collect();

    assert!(garcia_patients.len() >= 2);
}

#[wasm_bindgen_test]
async fn test_encounter_status_transitions() {
    use dental_intake::storage::{EncounterStore, PatientStore};

    // Create patient store which initializes the database
    let patient_store = PatientStore::new()
        .await
        .expect("Failed to create patient store");

    // Create encounter store using the same database
    let store = EncounterStore::from_db(patient_store.db.clone());

    let patient_id = Uuid::new_v4().to_string();
    let mut encounter = create_test_encounter(&patient_id);
    let encounter_id = encounter.id.clone().expect("Encounter should have ID");

    // Test status progression: Planned -> In Progress -> Finished
    encounter.status = EncounterStatus::Planned;
    store
        .save(&encounter)
        .await
        .expect("Failed to save planned encounter");

    let retrieved = store
        .get(&encounter_id)
        .await
        .expect("Failed to get encounter")
        .expect("Encounter not found");
    assert_eq!(retrieved.status, EncounterStatus::Planned);

    // Update to In Progress
    encounter.status = EncounterStatus::InProgress;
    store
        .save(&encounter)
        .await
        .expect("Failed to update to in-progress");

    let retrieved = store
        .get(&encounter_id)
        .await
        .expect("Failed to get encounter")
        .expect("Encounter not found");
    assert_eq!(retrieved.status, EncounterStatus::InProgress);

    // Update to Finished
    encounter.status = EncounterStatus::Finished;
    store
        .save(&encounter)
        .await
        .expect("Failed to update to finished");

    let retrieved = store
        .get(&encounter_id)
        .await
        .expect("Failed to get encounter")
        .expect("Encounter not found");
    assert_eq!(retrieved.status, EncounterStatus::Finished);
}
