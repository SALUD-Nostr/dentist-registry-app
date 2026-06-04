# Portal Salud — Dental Registry

A dental clinic management application and the **reference implementation of the
[SALUD protocol](https://github.com/SALUD-Nostr/salud-protocol)** — an open
standard for self-sovereign health records built on **FHIR** (for clinical data)
and **Nostr** (for cryptographic identity, authorship, and ownership).

Every record the app creates — patients, encounters, diagnoses, procedures,
intake forms — is a FHIR R4 resource wrapped in a signed Nostr event, owned by
the keypair that authored it. Records are stored locally first and remain
verifiable wherever they travel.

---

## Features

- **Patient registry** — register and edit patients with FHIR-compliant
  demographics.
- **Medical intake (Ficha Médica)** — capture each patient's key initial
  health questions, stored as structured clinical resources.
- **Clinical records** — conditions, procedures, allergies, and clinical
  impressions, each modeled as a FHIR resource.
- **Encounters & scheduling** — appointment booking and a clinic calendar.
- **Doctors & rooms** — manage providers and room schedules.
- **Self-sovereign identity** — sign in with a Nostr keypair; back up and
  recover your keys from a mnemonic phrase in Settings.
- **Local-first storage** — all data persists on-device in IndexedDB; no server
  account required.

---

## How it works

The app is a [Yew](https://yew.rs/) single-page application compiled to
WebAssembly. Its data model is the SALUD protocol:

```
FHIR R4 resource  ──serialize──▶  Nostr event (signed)  ──store──▶  IndexedDB
   (Patient,                       kind 82,                          (local-first)
    Condition,                     tag ["fhir", "<Type>"],
    Procedure, …)                  signed by the user's keypair
```

- **`salud-types`** — a small Rust crate providing minimal, R4-compliant FHIR
  resources (`Patient`, `Encounter`, `Condition`, `Procedure`,
  `AllergyIntolerance`, `ClinicalImpression`) with builders and serde support.
- **`dental_intake`** — the Yew frontend: UI, routing, IndexedDB storage, and
  the `SaludNote` helpers that wrap each FHIR resource in a signed Nostr event.

For the protocol itself, see
[SALUD-Nostr/salud-protocol](https://github.com/SALUD-Nostr/salud-protocol).

---

## Project layout

```
.
├── salud-types/      # FHIR R4 resource types (Rust library)
└── dental_intake/    # Yew/WASM frontend (the app)
    ├── src/          # components, features, storage, routing
    ├── tests/        # browser-based E2E tests (wasm-bindgen-test)
    ├── index.html
    ├── input.css     # Tailwind v4 entry
    └── Trunk.toml    # build/serve config
```

This is a Cargo workspace; `salud-types` and `dental_intake` are its members.

---

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024) with the
  `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/) — the WASM web bundler:
  ```bash
  cargo install trunk
  ```
- [Tailwind CSS v4](https://tailwindcss.com/) CLI available as `tailwindcss4`
  (invoked by Trunk's pre-build hook to generate `output.css`).

### Run the app

```bash
cd dental_intake
trunk serve
```

The app is served at <http://localhost:8002>. Trunk watches `src`, `index.html`,
and `input.css`, rebuilding on change.

### Build for production

```bash
cd dental_intake
trunk build --release
```

The bundled output is written to `dental_intake/dist/`.

### Run the tests

Browser-based end-to-end tests live in `dental_intake/tests/`. See
[`dental_intake/tests/README.md`](dental_intake/tests/README.md) for details:

```bash
cd dental_intake
wasm-pack test --headless --firefox
```

---

## License

[MIT](LICENSE) © Illuminodes
