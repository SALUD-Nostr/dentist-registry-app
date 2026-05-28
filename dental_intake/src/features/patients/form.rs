//! Patient registration form

use chrono::NaiveDate;
use gloo_console::log;
use salud_types::{
    Address, AdministrativeGender, ContactPoint, ContactPointSystem, ContactPointUse, HumanName,
    PatientBuilder,
};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typography::{Label, Subtitle, Title};
use crate::components::{Button, ButtonSize, ButtonVariant};

#[function_component(PatientForm)]
pub fn patient_form() -> Html {
    let navigator = use_navigator().unwrap();
    let patient_store = crate::storage::use_patient_store();
    let notify_patients_changed = crate::storage::use_notify_patients_changed();
    let nostr_key = nostr_minions::use_nostr_key();

    // Form state
    let given_name = use_state(String::new);
    let family_name = use_state(String::new);
    let birth_date = use_state(String::new);
    let gender = use_state(|| None::<AdministrativeGender>);
    let phone = use_state(String::new);
    let email = use_state(String::new);
    let street = use_state(String::new);
    let city = use_state(String::new);
    let state = use_state(String::new);
    let postal_code = use_state(String::new);
    let country = use_state(String::new);

    // Validation and UI state
    let errors = use_state(Vec::<String>::new);
    let is_saving = use_state(|| false);

    let validate_form = {
        let given_name = given_name.clone();
        let family_name = family_name.clone();
        let errors = errors.clone();

        move || {
            let mut validation_errors = Vec::new();

            if given_name.trim().is_empty() {
                validation_errors.push("El nombre es requerido".to_string());
            }

            if family_name.trim().is_empty() {
                validation_errors.push("El apellido es requerido".to_string());
            }

            errors.set(validation_errors.clone());
            validation_errors.is_empty()
        }
    };

    let handle_submit = {
        let navigator = navigator.clone();
        let given_name = given_name.clone();
        let family_name = family_name.clone();
        let birth_date = birth_date.clone();
        let gender = gender.clone();
        let phone = phone.clone();
        let email = email.clone();
        let street = street.clone();
        let city = city.clone();
        let state = state.clone();
        let postal_code = postal_code.clone();
        let country = country.clone();
        let is_saving = is_saving.clone();
        let errors = errors.clone();
        let nostr_key = nostr_key.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if !validate_form() {
                return;
            }

            is_saving.set(true);
            errors.set(Vec::new());

            let navigator = navigator.clone();
            let patient_store = patient_store.clone();
            let given_name_val = (*given_name).clone();
            let family_name_val = (*family_name).clone();
            let birth_date_val = (*birth_date).clone();
            let gender_val = (*gender).clone();
            let phone_val = (*phone).clone();
            let email_val = (*email).clone();
            let street_val = (*street).clone();
            let city_val = (*city).clone();
            let state_val = (*state).clone();
            let postal_code_val = (*postal_code).clone();
            let country_val = (*country).clone();
            let is_saving = is_saving.clone();
            let errors = errors.clone();
            let notify_patients_changed = notify_patients_changed.clone();
            let nostr_key = nostr_key.clone();

            // Captured for resetting form state after a successful save, so the
            // next "Nuevo Paciente" starts blank instead of showing stale data.
            let given_name_reset = given_name.clone();
            let family_name_reset = family_name.clone();
            let birth_date_reset = birth_date.clone();
            let gender_reset = gender.clone();
            let phone_reset = phone.clone();
            let email_reset = email.clone();
            let street_reset = street.clone();
            let city_reset = city.clone();
            let state_reset = state.clone();
            let postal_code_reset = postal_code.clone();
            let country_reset = country.clone();

            spawn_local(async move {
                // Build HumanName
                let human_name = HumanName {
                    text: Some(format!("{given_name_val} {family_name_val}")),
                    family: Some(family_name_val),
                    given: Some(vec![given_name_val]),
                };

                // Build ContactPoints
                let mut telecom = Vec::new();
                if !phone_val.trim().is_empty() {
                    telecom.push(ContactPoint {
                        system: ContactPointSystem::Phone,
                        value: phone_val,
                        use_: Some(ContactPointUse::Mobile),
                    });
                }
                if !email_val.trim().is_empty() {
                    telecom.push(ContactPoint {
                        system: ContactPointSystem::Email,
                        value: email_val,
                        use_: None,
                    });
                }

                // Build Address
                let address = if !street_val.trim().is_empty()
                    || !city_val.trim().is_empty()
                    || !state_val.trim().is_empty()
                    || !postal_code_val.trim().is_empty()
                    || !country_val.trim().is_empty()
                {
                    let mut lines = Vec::new();
                    if !street_val.trim().is_empty() {
                        lines.push(street_val.clone());
                    }

                    Some(vec![Address {
                        text: None,
                        line: if lines.is_empty() { None } else { Some(lines) },
                        city: if city_val.is_empty() {
                            None
                        } else {
                            Some(city_val)
                        },
                        state: if state_val.is_empty() {
                            None
                        } else {
                            Some(state_val)
                        },
                        postal_code: if postal_code_val.is_empty() {
                            None
                        } else {
                            Some(postal_code_val)
                        },
                        country: if country_val.is_empty() {
                            None
                        } else {
                            Some(country_val)
                        },
                    }])
                } else {
                    None
                };

                // Parse birth date
                let birth_date_parsed = if !birth_date_val.is_empty() {
                    match NaiveDate::parse_from_str(&birth_date_val, "%Y-%m-%d") {
                        Ok(date) => Some(date),
                        Err(e) => {
                            log!("Error parsing birth date:", format!("{:?}", e));
                            None
                        }
                    }
                } else {
                    None
                };

                // Build Patient using builder
                let mut builder = PatientBuilder::default();
                builder
                    .id(Uuid::new_v4().to_string())
                    .resource_type("Patient".to_string())
                    .active(true)
                    .name(vec![human_name]);

                if !telecom.is_empty() {
                    builder.telecom(telecom);
                }

                if let Some(g) = gender_val {
                    builder.gender(g);
                }

                if let Some(bd) = birth_date_parsed {
                    builder.birth_date(bd);
                }

                if let Some(addr) = address {
                    builder.address(addr);
                }

                let patient_result = builder.build();

                match patient_result {
                    Ok(patient) => {
                        // Get keypair
                        let keypair = match nostr_key.as_ref() {
                            Some(key) => key,
                            None => {
                                log!("Error: No keypair available");
                                errors.set(vec!["No se encontró la clave de firma".to_string()]);
                                is_saving.set(false);
                                return;
                            }
                        };

                        log!("Saving patient:", format!("{:?}", patient));
                        match patient_store.save(&patient, keypair).await {
                            Ok(()) => {
                                log!("Patient saved successfully");
                                // Clear the form so a subsequent "Nuevo Paciente"
                                // does not show the patient we just created.
                                given_name_reset.set(String::new());
                                family_name_reset.set(String::new());
                                birth_date_reset.set(String::new());
                                gender_reset.set(None);
                                phone_reset.set(String::new());
                                email_reset.set(String::new());
                                street_reset.set(String::new());
                                city_reset.set(String::new());
                                state_reset.set(String::new());
                                postal_code_reset.set(String::new());
                                country_reset.set(String::new());
                                errors.set(Vec::new());
                                is_saving.set(false);
                                notify_patients_changed.emit(());
                                navigator.push(&crate::router::Route::PatientsList);
                            }
                            Err(e) => {
                                log!("Error saving patient:", format!("{:?}", e));
                                errors.set(vec![format!("Error al guardar: {:?}", e)]);
                                is_saving.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        log!("Error building patient:", format!("{:?}", e));
                        errors.set(vec![format!("Error al crear paciente: {}", e)]);
                        is_saving.set(false);
                    }
                }
            });
        })
    };

    let handle_cancel = {
        let navigator = navigator;
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::PatientsList);
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 md:p-8 overflow-auto">
            <div class="max-w-3xl mx-auto w-full">
                <div class="mb-6">
                    <button
                        onclick={handle_cancel.clone()}
                        class="flex items-center gap-2 text-muted hover:text-foreground transition-colors mb-4"
                    >
                        <crate::components::ArrowLeft class="size-5" />
                        {"Volver a Pacientes"}
                    </button>
                    <Title>{"Registrar Nuevo Paciente"}</Title>
                </div>

                <form onsubmit={handle_submit}>
                    <shady_minions::ui::Card class="!border-0 !shadow-none">
                        <div class="space-y-6">
                            // Error messages
                            if !(*errors).is_empty() {
                                <div class="p-4 bg-red-50 border border-red-200 rounded-lg">
                                    <div class="flex items-start gap-2">
                                        <crate::components::X class="size-5 text-red-600 mt-0.5" />
                                        <div class="flex-1">
                                            <Subtitle class="font-semibold text-red-900 mb-1">{"Errores de validación"}</Subtitle>
                                            <ul class="list-disc list-inside text-sm text-red-700">
                                                { for (*errors).iter().map(|error| html! {
                                                    <li>{error}</li>
                                                }) }
                                            </ul>
                                        </div>
                                    </div>
                                </div>
                            }

                            // Personal Information Section
                            <div>
                                <Subtitle class="mb-4">{"Información Personal"}</Subtitle>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div>
                                        <Label class="block mb-2">
                                            {"Nombre"}<span class="text-red-500">{"*"}</span>
                                        </Label>
                                        <input
                                            type="text"
                                            value={(*given_name).clone()}
                                            oninput={
                                                let given_name = given_name.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    given_name.set(input.value());
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                            placeholder="Juan"
                                        />
                                    </div>

                                    <div>
                                        <Label class="block mb-2">
                                            {"Apellido"}<span class="text-red-500">{"*"}</span>
                                        </Label>
                                        <input
                                            type="text"
                                            value={(*family_name).clone()}
                                            oninput={
                                                let family_name = family_name.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    family_name.set(input.value());
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                            placeholder="Pérez"
                                        />
                                    </div>

                                    <div>
                                        <Label class="block mb-2">
                                            {"Fecha de Nacimiento"}
                                        </Label>
                                        <input
                                            type="date"
                                            value={(*birth_date).clone()}
                                            oninput={
                                                let birth_date = birth_date.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    birth_date.set(input.value());
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                        />
                                    </div>

                                    <div>
                                        <Label class="block mb-2">
                                            {"Género"}
                                        </Label>
                                        <select
                                            value={match *gender {
                                                Some(AdministrativeGender::Male) => "male",
                                                Some(AdministrativeGender::Female) => "female",
                                                Some(AdministrativeGender::Other) => "other",
                                                Some(AdministrativeGender::Unknown) | None => "",
                                            }}
                                            onchange={
                                                let gender = gender.clone();
                                                Callback::from(move |e: Event| {
                                                    let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                                    let value = match select.value().as_str() {
                                                        "male" => Some(AdministrativeGender::Male),
                                                        "female" => Some(AdministrativeGender::Female),
                                                        "other" => Some(AdministrativeGender::Other),
                                                        _ => None,
                                                    };
                                                    gender.set(value);
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                        >
                                            <option value="">{"Seleccionar..."}</option>
                                            <option value="male">{"Masculino"}</option>
                                            <option value="female">{"Femenino"}</option>
                                            <option value="other">{"Otro"}</option>
                                        </select>
                                    </div>
                                </div>
                            </div>

                            // Contact Information Section
                            <div>
                                <Subtitle class="mb-4">{"Información de Contacto"}</Subtitle>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div>
                                        <Label class="block mb-2">
                                            {"Teléfono"}
                                        </Label>
                                        <input
                                            type="tel"
                                            value={(*phone).clone()}
                                            oninput={
                                                let phone = phone.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    phone.set(input.value());
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                            placeholder="+503 7000-0000"
                                        />
                                    </div>

                                    <div>
                                        <Label class="block mb-2">
                                            {"Email"}
                                        </Label>
                                        <input
                                            type="email"
                                            value={(*email).clone()}
                                            oninput={
                                                let email = email.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    email.set(input.value());
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                            placeholder="ejemplo@email.com"
                                        />
                                    </div>
                                </div>
                            </div>

                            // Address Section
                            <div>
                                <Subtitle class="mb-4">{"Dirección"}</Subtitle>
                                <div class="space-y-4">
                                    <div>
                                        <Label class="block mb-2">
                                            {"Calle"}
                                        </Label>
                                        <input
                                            type="text"
                                            value={(*street).clone()}
                                            oninput={
                                                let street = street.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                    street.set(input.value());
                                                })
                                            }
                                            class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                            placeholder="Calle Principal 123"
                                        />
                                    </div>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        <div>
                                            <Label class="block mb-2">
                                                {"Ciudad"}
                                            </Label>
                                            <input
                                                type="text"
                                                value={(*city).clone()}
                                                oninput={
                                                    let city = city.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        city.set(input.value());
                                                    })
                                                }
                                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                                placeholder="San Salvador"
                                            />
                                        </div>

                                        <div>
                                            <Label class="block mb-2">
                                                {"Departamento"}
                                            </Label>
                                            <input
                                                type="text"
                                                value={(*state).clone()}
                                                oninput={
                                                    let state = state.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        state.set(input.value());
                                                    })
                                                }
                                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                                placeholder="San Salvador"
                                            />
                                        </div>
                                    </div>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        <div>
                                            <Label class="block mb-2">
                                                {"Código Postal"}
                                            </Label>
                                            <input
                                                type="text"
                                                value={(*postal_code).clone()}
                                                oninput={
                                                    let postal_code = postal_code.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        postal_code.set(input.value());
                                                    })
                                                }
                                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                                placeholder="1101"
                                            />
                                        </div>

                                        <div>
                                            <Label class="block mb-2">
                                                {"País"}
                                            </Label>
                                            <input
                                                type="text"
                                                value={(*country).clone()}
                                                oninput={
                                                    let country = country.clone();
                                                    Callback::from(move |e: InputEvent| {
                                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                        country.set(input.value());
                                                    })
                                                }
                                                class="w-full px-3 py-2 border border-muted rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                                                placeholder="El Salvador"
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Form Actions
                            <div class="flex justify-end gap-3 pt-4 border-t border-muted">
                                <Button
                                    variant={ButtonVariant::Outline}
                                    size={ButtonSize::Medium}
                                    onclick={Some(handle_cancel)}
                                    disabled={*is_saving}
                                >
                                    {"Cancelar"}
                                </Button>
                                <Button
                                    variant={ButtonVariant::Primary}
                                    size={ButtonSize::Medium}
                                    button_type={"submit".to_string()}
                                    loading={*is_saving}
                                    disabled={*is_saving}
                                >
                                    if !*is_saving {
                                        <crate::components::Check class="size-5" />
                                    }
                                    {if *is_saving { "Guardando..." } else { "Guardar Paciente" }}
                                </Button>
                            </div>
                        </div>
                    </shady_minions::ui::Card>
                </form>
            </div>
        </div>
    }
}
