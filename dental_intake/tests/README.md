# End-to-End Tests for Portal Salud

This directory contains browser-based E2E tests using `wasm-bindgen-test`.

## Prerequisites

1. Install `wasm-pack`:
```bash
cargo install wasm-pack
```

2. Ensure you have Chrome or Firefox installed (for headless testing).

## Running Tests

### Run all tests in Firefox (headless) - RECOMMENDED ✅:
```bash
cd dental_intake
wasm-pack test --headless --firefox
```

### Run all tests in Chrome (headless) - Has issues:
```bash
wasm-pack test --headless --chrome
# Note: Chrome has ChromeDriver issues, use Firefox instead
```

### Run tests with browser UI (for debugging):
```bash
wasm-pack test --chrome
```

### Run a specific test file:
```bash
wasm-pack test --headless --chrome --test storage_tests
```

## Test Structure

### Storage Tests (`storage_tests.rs`)
Tests the IndexedDB storage layer for FHIR resources:

- **Patient Store CRUD**: Create, Read, Update, Delete operations
- **Encounter Store CRUD**: Full lifecycle with patient linking
- **Clinical Impression Store CRUD**: Full lifecycle with encounter linking
- **Data Persistence**: Verify data survives store recreation
- **Search**: Patient search by name
- **Status Transitions**: Encounter status workflow

## Writing New Tests

1. Create a new `.rs` file in the `tests/` directory
2. Add `wasm_bindgen_test_configure!(run_in_browser);` at the top
3. Import necessary types from `dental_intake` crate
4. Write test functions with `#[wasm_bindgen_test]` attribute

Example:
```rust
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_my_feature() {
    use dental_intake::storage::PatientStore;

    let store = PatientStore::new().await.unwrap();
    // Your test code here
    assert!(true);
}
```

## CI/CD Integration

To run tests in CI/CD pipelines:

```yaml
# Example GitHub Actions
- name: Install wasm-pack
  run: cargo install wasm-pack

- name: Run E2E tests
  run: |
    cd dental_intake
    wasm-pack test --headless --chrome
```

## Troubleshooting

### Tests fail to start
- Ensure Chrome/Firefox is installed
- Check that wasm-pack is installed: `wasm-pack --version`
- Try running with `--chrome` (no headless) to see browser errors

### IndexedDB errors
- Clear browser cache/IndexedDB data
- Each test should use unique IDs to avoid conflicts
- Tests run in isolated browser contexts

### Module not found errors
- Ensure `src/lib.rs` exists and exports necessary modules
- Check that Cargo.toml has `[lib]` configuration
- Verify imports in test files match module structure

## Test Coverage

Current coverage:
- ✅ Storage layer (CRUD operations)
- ⏳ Patient management UI (planned)
- ⏳ Encounter management UI (planned)
- ⏳ Clinical impression UI (planned)

## Notes

- Tests run in a real browser environment with full Web APIs
- IndexedDB operations are tested with actual browser storage
- Tests are asynchronous and use `async/await`
- Each test should be independent and clean up after itself
