# Nostr Authentication Re-enablement Strategy

**Last Updated:** 2026-01-23  
**Purpose:** Document the strategy for re-enabling Nostr-based authentication from legacy code into the current FHIR-compliant implementation  

---

## 🎯 Current State Analysis

### What's Currently Working
- ✅ **FHIR-compliant data models** - Patient, Encounter, ClinicalImpression with proper serialization
- ✅ **Complete UI implementation** - Patient registration, encounter scheduling, clinical impressions
- ✅ **IndexedDB storage layer** - Local persistence with no external dependencies
- ✅ **Modern Yew architecture** - Clean separation of concerns, hooks-based state management

### What's Commented Out (Legacy Nostr Code)
The following authentication-related code is currently disabled:

#### In `src/main.rs`:
```rust
// Lines 59-108: Complete Nostr messaging system
// - use_send_server_message() hook
// - encrypt_server_message() function 
// - NIP44 encryption support
// - PARAVIDA_PUBKEY integration
```

#### In `src/features/mod.rs`:
```rust
// Lines 15-16: Core authentication modules
// pub mod login;
// pub mod nostr_notes;
```

#### In `src/router/mod.rs`:
```rust
// Lines 37-41: Sync status integration
// let sync_status = crate::features::nostr_notes::use_sync_status();
```

### Current Authentication Flow
**Current State:** No authentication - app runs as local-only
- Login page exists but is not integrated into router
- Nostr key management is available but unused
- App accessible without any credentials

---

## 🔄 Re-enablement Strategy

### Phase 1: Core Authentication Restoration

#### 1.1 Restore Login Module Integration
**File:** `src/features/mod.rs`
```rust
// Uncomment line 15:
pub mod login;
```

**Impact:** Enables the existing Nostr-based login system

#### 1.2 Integrate Login Wrapper
**File:** `src/router/mod.rs` - Add to `AppRouter` component
```rust
// Wrap the entire app with LoginWrapper
#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <features::login::LoginWrapper>
            <div class="flex h-[100vh] w-[100vw] flex-col-reverse md:flex-row">
                // existing router content
            </div>
        </features::login::LoginWrapper>
    }
}
```

---

## 🔧 Technical Implementation Details

### Current Login System Architecture

#### Login Provider (`features/login/provider.rs`)
- Uses `nostr_minions::use_nostr_key()` hook
- Renders children only when key is present
- Falls back to `LoginPage` when no key exists

#### Login Page (`features/login/login_page.rs`)
- Accepts `nsec` (Nostr secret key) input
- Uses `nostr_minions::use_create_local_key()` hook
- Parses nsec into `NostrKeypair`
- Sets key as extractable for browser storage

### Integration Points with New FHIR System

#### 1. Storage Layer Compatibility
**Current:** IndexedDB stores FHIR resources directly  
**Legacy:** IndexedDB stored Nostr notes containing data  

**Solution:** 

Add methods to FHIR resources to create Nostr notes:

- d tag with resource id 
- p tag with user's public key 
- fhir tag with resource type

Add methods to try parse nostr notes into FHIR resources:

- Use fhir tag to determine resource type if needed
- Use p tag to determine user's public key if needed

Change stores to store Nostr notes instead of FHIR resources:

- Store notes in IndexedDB
- Update store getters to parse notes into resources

