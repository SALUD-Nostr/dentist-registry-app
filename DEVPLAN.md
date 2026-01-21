# DEVPLAN

## FHIR Resources 

- Patient - https://fhir.hl7.org/fhir/patient.html
- Encounter - https://fhir.hl7.org/fhir/encounter.html
- ClinicalImpression - https://fhir.hl7.org/fhir/clinicalimpression.html

## Goals 

- Register patients
- Schedule encounters for patients 
- Record clinical impressions

## Screens 

- Patient intake form 
- Patient List
- Patient Detail

- Encounter intake form
- Encounter Schedule (calendar)

- Clinical Impression intake form (active encounter)
- Encounter History (completed encounter list)
- Encounter Detail (clinical impressions)


## Current project status

Similar application already built out with Yew and WASM at `dental_intake/` directory.
Can refactor the app to currrent usecases, keeping the same UI patterns and styles, as
well as technologies for storage/transmission/authentication.


## Tasks


### Data Model 

- Create structs in `salud-types` for the before mentioned compliant FHIR resources
- Ensure each struct is serde serializable
- Ensure each struct has a builder pattern 


### App refactor

- Refactor the `dental_intake` app to remove routes of old use cases and create new routes for the new usecases
- Remove dependencies on `paravida-models` and `paravida-components`
- Keep same UI patterns and styles

### Storage 
 
- Modify the IDB abstractions and methods in `dental_intake` to use the `salud-types` structs


### UI 

Refactor the app screens to fit current use cases
