# Route Refactoring Plan

## Current Routes vs. New Routes

### Current Routes (dental_intake)
| Route | Purpose | Maps To |
|-------|---------|---------|
| `/` | Home/Dashboard | ✅ Keep as Dashboard |
| `/calendar` | Calendar view | ✅ Repurpose as "Encounter Schedule" |
| `/history` | Appointment history | ✅ Repurpose as "Encounter History" |
| `/admin-appointment` | Create appointment | ✅ Repurpose as "Schedule Encounter" |
| `/appointment/:id` | Appointment detail | ✅ Repurpose as "Encounter Detail" |
| `/doctors` | Doctors list | ❌ Remove (not in DEVPLAN) |
| `/doctors/:pubkey` | Doctor detail | ❌ Remove (not in DEVPLAN) |
| `/create-doctor` | Create doctor | ❌ Remove (not in DEVPLAN) |

### New Routes (DEVPLAN Compliant)

| Route | Purpose | DEVPLAN Screen | Status |
|-------|---------|----------------|--------|
| `/` | Dashboard | - | Existing (needs update) |
| `/patients` | Patient list | Patient List | **NEW** |
| `/patients/new` | Register new patient | Patient intake form | **NEW** |
| `/patients/:id` | Patient detail view | Patient Detail | **NEW** |
| `/encounters` | Encounter schedule (calendar) | Encounter Schedule | Refactor from `/calendar` |
| `/encounters/new` | Schedule new encounter | Encounter intake form | Refactor from `/admin-appointment` |
| `/encounters/:id` | Encounter detail with impressions | Encounter Detail | Refactor from `/appointment/:id` |
| `/encounters/:id/impression/new` | Record clinical impression | Clinical Impression intake form | **NEW** |
| `/history` | Completed encounters list | Encounter History | Existing (needs update) |

## Implementation Strategy

### Phase 1: Add salud-types dependency to dental_intake
- Update `dental_intake/Cargo.toml` to include `salud-types`
- Ensure it compiles

### Phase 2: Create UI Component Library
Create replacement components for `paravida-components` in `dental_intake/src/components/`:
- `icons.rs` - Icon components (Home, Calendar, List, Plus, User, etc.)
- `logo.rs` - App logo
- `loader.rs` - Loading spinner
- `card.rs` - Card component
- `typography.rs` - Text components

### Phase 3: Update Router
- Define new route enum in `router/mod.rs`
- Update navbar with new navigation items
- Remove doctor-related routes

### Phase 4: Update Storage Layer
- Refactor `local_db/mod.rs` to use `salud-types` instead of `paravida-models`
- Update object stores for Patient, Encounter, ClinicalImpression
- Update all storage methods

### Phase 5: Create/Refactor Feature Modules
- `features/patients/` (NEW)
  - `list.rs` - Patient list view
  - `form.rs` - Patient intake form
  - `detail.rs` - Patient detail view

- `features/encounters/` (refactor from admin_appointment + calendar)
  - `schedule.rs` - Calendar view
  - `form.rs` - Encounter intake form
  - `detail.rs` - Encounter detail view
  - `history.rs` - Completed encounters

- `features/clinical_impressions/` (NEW)
  - `form.rs` - Clinical impression intake form

### Phase 6: Update Main App
- Remove paravida-components imports
- Use new component library
- Update providers if needed

## Questions to Resolve

1. Should we keep a practitioners/doctors section even though it's not in DEVPLAN?
   - If yes, where should practitioners be managed?
   - If no, how do we handle the practitioner reference in Encounters?

2. Should the home/dashboard show:
   - Summary statistics?
   - Recent patients?
   - Today's encounters?
   - Quick actions?
