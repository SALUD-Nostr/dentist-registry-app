use yew::prelude::*;

#[function_component(CreateDoctorPage)]
pub fn create_doctor_page() -> Html {
    let navigator = yew_router::hooks::use_navigator();
    let submission_errors = use_state(Vec::<String>::new);
    let practitioner = use_mut_ref(paravida_models::ParavidaPractitioner::default);
    let show_preview = use_state(|| false);
    let send_server_message = crate::use_send_server_message();
    let is_submitting = use_state(|| false);
    let doctors_ctx = crate::features::nostr_notes::use_doctors();
    let initial_doctor_count =
        use_state(|| doctors_ctx.as_ref().map_or(0, |ctx| ctx.doctors.len()));

    let validate_practitioner = {
        let practitioner = practitioner.clone();
        let submission_errors = submission_errors.setter();
        let show_preview = show_preview.setter();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let practitioner = practitioner.borrow();
            let errors = validate_practitioner_profile(&practitioner);
            let is_valid = errors.is_empty();
            submission_errors.set(errors);
            if is_valid {
                show_preview.set(true);
            }
            // Form state is preserved in practitioner RefCell, so no reset on error
        })
    };

    let close_preview = {
        let show_preview = show_preview.setter();
        Callback::from(move |_| show_preview.set(false))
    };

    let confirm = {
        let practitioner = practitioner.clone();
        let show_preview = show_preview.setter();
        let send_server_message = send_server_message.clone();
        let is_submitting = is_submitting.setter();
        Callback::from(move |_| {
            let mut practitioner_data = practitioner.borrow_mut().clone();
            let Some(dui) = practitioner_data
                .identification
                .iter_mut()
                .find(|i| i.system == paravida_models::IdentifierType::Dui)
                .map(|i| i.value.clone())
            else {
                return;
            };
            practitioner_data
                .contacts
                .push(paravida_models::PractitionerContact {
                    system: paravida_models::PractitionerContactSystem::Nostr,
                    value: dui,
                    work: true,
                });

            // Create server message with practitioner data
            let server_message = nostr_minions::nostro2::NostrNote {
                content: serde_json::to_string(&practitioner_data).unwrap_or_default(),
                kind: crate::constants::magic_numbers::ADMIN_PROFILE_REQUEST_KIND,
                ..Default::default()
            };

            // Send the message to the server
            send_server_message.emit(server_message);

            // Close preview and show submitting state
            show_preview.set(false);
            is_submitting.set(true);
        })
    };

    let on_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Some(navigator) = navigator.as_ref() {
                navigator.back();
            }
        })
    };

    // Watch for the new doctor to appear in the doctors list
    {
        let navigator = navigator.clone();
        let is_submitting = is_submitting.clone();
        let initial_count = *initial_doctor_count;
        use_effect_with(
            (doctors_ctx.clone(), is_submitting.clone()),
            move |(doctors_ctx, is_submitting)| {
                if !**is_submitting {
                    return;
                }

                if let Some(ctx) = doctors_ctx {
                    let current_count = ctx.doctors.len();
                    // Check if a new doctor was added
                    if current_count > initial_count {
                        // Navigate back to admin appointment page
                        if let Some(navigator) = navigator.as_ref() {
                            navigator.push(&crate::router::AppRoute::AdminAppointment);
                        }
                    }
                }
            },
        );
    }

    html! {
        <div class="size-full overflow-y-auto p-4 md:p-6">
            <div class="max-w-4xl mx-auto">
                <div class="flex flex-row items-center justify-between mb-6">
                    <paravida_components::typography::Highlight>
                        {"Crear Nuevo Perfil de Doctor"}
                    </paravida_components::typography::Highlight>
                    {if *is_submitting {
                        html! {}
                    } else {
                        html! {
                            <paravida_components::buttons::OutlineButton
                                onclick={on_back}
                                class="!w-auto !px-6"
                            >
                                <span class="flex items-center gap-2">
                                    <paravida_components::icons::ChevronLeft class="size-4" />
                                    {"Volver"}
                                </span>
                            </paravida_components::buttons::OutlineButton>
                        }
                    }}
                </div>

                {if *is_submitting {
                    html! {
                        <paravida_components::Card>
                            <div class="flex flex-col gap-6 items-center justify-center py-12">
                                <svg
                                    class="size-16 text-secondary animate-spin"
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                >
                                    <circle
                                        class="opacity-25"
                                        cx="12"
                                        cy="12"
                                        r="10"
                                        stroke="currentColor"
                                        stroke-width="4"
                                    />
                                    <path
                                        class="opacity-75"
                                        fill="currentColor"
                                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                    />
                                </svg>
                                <paravida_components::typography::Highlight class="text-center">
                                    {"Publicando perfil de doctor..."}
                                </paravida_components::typography::Highlight>
                                <paravida_components::typography::P class="text-center text-muted">
                                    {"El servidor está procesando la solicitud y publicando el perfil. Esto puede tomar unos momentos."}
                                </paravida_components::typography::P>
                            </div>
                        </paravida_components::Card>
                    }
                } else {
                    html! {
                        <paravida_components::Card>
                            <form onsubmit={validate_practitioner} class="space-y-4">
                                <DoctorNameInputs practitioner={practitioner.clone()} />
                                <DoctorSpecialtyInputs practitioner={practitioner.clone()} />
                                <DoctorContactInputs practitioner={practitioner.clone()} />
                                <DoctorIdentityInputs practitioner={practitioner.clone()} />

                                <div class="flex flex-row gap-2 w-full items-center justify-center pt-4">
                                    <paravida_components::buttons::NormalButton
                                        r#type="submit"
                                    >
                                        { "Validar Perfil" }
                                    </paravida_components::buttons::NormalButton>
                                </div>

                                {(!submission_errors.is_empty()).then(|| html! {
                                    <div class="mt-4">
                                        <paravida_components::alerts::FormErrors errors={(*submission_errors).clone()} />
                                    </div>
                                })}
                            </form>
                        </paravida_components::Card>
                    }
                }}

                <shady_minions::ui::Modal is_open={show_preview.clone()}>
                    <div class="flex flex-col gap-6 max-w-xl w-full">
                        <DoctorProfilePreview profile={practitioner.borrow().clone()} />
                        <div class="flex flex-row gap-3 w-full items-center justify-center">
                            <paravida_components::buttons::NormalButton
                                onclick={close_preview}
                            >
                                { "Regresar" }
                            </paravida_components::buttons::NormalButton>
                            <paravida_components::buttons::SecondaryButton
                                onclick={confirm}
                            >
                                { "Confirmar Perfil" }
                            </paravida_components::buttons::SecondaryButton>
                        </div>
                    </div>
                </shady_minions::ui::Modal>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DoctorInputsProps {
    #[prop_or_default]
    pub practitioner: std::rc::Rc<std::cell::RefCell<paravida_models::ParavidaPractitioner>>,
}

#[function_component(DoctorNameInputs)]
pub fn doctor_name_inputs(props: &DoctorInputsProps) -> Html {
    let suffix_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                practitioner.borrow_mut().name.suffix = value;
            }
        })
    };
    let given_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                practitioner.borrow_mut().name.given = value;
            }
        })
    };
    let family_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                practitioner.borrow_mut().name.family = value;
            }
        })
    };

    html! {
        <div class="flex flex-row gap-2">
            <div class="flex flex-col gap-1 max-w-12 sm:max-w-16 md:max-w-24">
                <paravida_components::inputs::Label >{ "Titulo" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    required=false
                    placeholder="Dr."
                    r#type="text"
                    value={props.practitioner.borrow().name.suffix.clone()}
                    oninput={suffix_change}
                />
            </div>
            <div class="flex flex-col gap-1 flex-1">
                <paravida_components::inputs::Label >{ "Nombre" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    placeholder="John"
                    r#type="text"
                    value={props.practitioner.borrow().name.given.clone()}
                    oninput={given_change}
                />
            </div>
            <div class="flex flex-col gap-1 flex-1">
                <paravida_components::inputs::Label >{ "Apellido" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    placeholder="McGuire"
                    r#type="text"
                    value={props.practitioner.borrow().name.family.clone()}
                    oninput={family_change}
                />
            </div>
        </div>
    }
}

#[function_component(DoctorSpecialtyInputs)]
pub fn doctor_specialty_inputs(props: &DoctorInputsProps) -> Html {
    let onchange_specialty = {
        let practitioner = props.practitioner.clone();
        Callback::from(
            move |specialty: Option<paravida_models::ParavidaSpecialty>| {
                if let Some(specialty) = specialty {
                    practitioner.borrow_mut().specialty = vec![specialty];
                }
            },
        )
    };
    html! {
        <>
            <div class="flex flex-col gap-1 flex-1 w-full">
                <paravida_components::inputs::Label >{ "Especialidad" }</paravida_components::inputs::Label>
                <shady_minions::ui::Select<paravida_models::ParavidaSpecialty>
                    class="flex-1 w-full"
                    onchange={onchange_specialty}
                    required=true placeholder="Especialidad">
                    <shady_minions::ui::SelectTrigger<paravida_models::ParavidaSpecialty> label="Escoger Especialidad" />
                    <shady_minions::ui::SelectContent<paravida_models::ParavidaSpecialty>>
                        { for paravida_models::ParavidaSpecialty::all().iter().map(|specialty| {
                            html! {
                                <shady_minions::ui::SelectItem<paravida_models::ParavidaSpecialty> value={*specialty} label={specialty.display_text()} />
                            }
                        })}
                    </shady_minions::ui::SelectContent<paravida_models::ParavidaSpecialty>>
                </shady_minions::ui::Select<paravida_models::ParavidaSpecialty>>
            </div>
        </>
    }
}

#[function_component(DoctorContactInputs)]
pub fn doctor_contact_inputs(props: &DoctorInputsProps) -> Html {
    let email_value = props
        .practitioner
        .borrow()
        .contacts
        .iter()
        .find(|c| c.system == paravida_models::PractitionerContactSystem::Email)
        .map(|c| c.value.clone())
        .unwrap_or_default();

    let phone_value = props
        .practitioner
        .borrow()
        .contacts
        .iter()
        .find(|c| c.system == paravida_models::PractitionerContactSystem::Phone)
        .map(|c| c.value.clone())
        .unwrap_or_default();

    let email_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                if let Some(val) = practitioner
                    .borrow_mut()
                    .contacts
                    .iter_mut()
                    .find(|contact| {
                        contact.system == paravida_models::PractitionerContactSystem::Email
                    })
                {
                    val.value = value;
                } else {
                    practitioner
                        .borrow_mut()
                        .contacts
                        .push(paravida_models::PractitionerContact {
                            system: paravida_models::PractitionerContactSystem::Email,
                            value,
                            work: false,
                        });
                }
            }
        })
    };
    let phone_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                if let Some(val) = practitioner
                    .borrow_mut()
                    .contacts
                    .iter_mut()
                    .find(|contact| {
                        contact.system == paravida_models::PractitionerContactSystem::Phone
                    })
                {
                    val.value = value;
                } else {
                    practitioner
                        .borrow_mut()
                        .contacts
                        .push(paravida_models::PractitionerContact {
                            system: paravida_models::PractitionerContactSystem::Phone,
                            value,
                            work: false,
                        });
                }
            }
        })
    };

    html! {
        <div class="flex flex-row gap-2 w-full">
            <div class="flex flex-col gap-1 flex-1">
                <paravida_components::inputs::Label >{ "Email" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    placeholder="test@test.com"
                    r#type="email"
                    value={email_value}
                    oninput={email_change}
                />
            </div>
            <div class="flex flex-col gap-1 flex-1">
                <paravida_components::inputs::Label >{ "Teléfono" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    placeholder="+123456789"
                    r#type="text"
                    value={phone_value}
                    oninput={phone_change}
                />
            </div>
        </div>
    }
}

#[function_component(DoctorIdentityInputs)]
pub fn doctor_identity_inputs(props: &DoctorInputsProps) -> Html {
    let dui_value = props
        .practitioner
        .borrow()
        .identification
        .iter()
        .find(|i| i.system == paravida_models::IdentifierType::Dui)
        .map(|i| i.value.clone())
        .unwrap_or_default();

    let jvpm_value = props
        .practitioner
        .borrow()
        .identification
        .iter()
        .find(|i| i.system == paravida_models::IdentifierType::Jvpm)
        .map(|i| i.value.clone())
        .unwrap_or_default();

    let dui_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                if let Some(val) =
                    practitioner
                        .borrow_mut()
                        .identification
                        .iter_mut()
                        .find(|identification| {
                            identification.system == paravida_models::IdentifierType::Dui
                        })
                {
                    val.value = value;
                } else {
                    practitioner.borrow_mut().identification.push(
                        paravida_models::PractitionerIdentification {
                            system: paravida_models::IdentifierType::Dui,
                            value,
                        },
                    );
                }
            }
        })
    };
    let jvpm_change = {
        let practitioner = props.practitioner.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(value) = event
                .target()
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|target| target.value())
            {
                if let Some(val) =
                    practitioner
                        .borrow_mut()
                        .identification
                        .iter_mut()
                        .find(|identification| {
                            identification.system == paravida_models::IdentifierType::Jvpm
                        })
                {
                    val.value = value;
                } else {
                    practitioner.borrow_mut().identification.push(
                        paravida_models::PractitionerIdentification {
                            system: paravida_models::IdentifierType::Jvpm,
                            value,
                        },
                    );
                }
            }
        })
    };
    html! {
        <div class="flex flex-row gap-2 w-full">
            <div class="flex flex-col gap-1 flex-1">
                <paravida_components::inputs::Label >{ "DUI" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    placeholder="00000000-0"
                    r#type="text"
                    value={dui_value}
                    oninput={dui_change}
                />
            </div>
            <div class="flex flex-col gap-1 flex-1">
                <paravida_components::inputs::Label >{ "JVPM" }</paravida_components::inputs::Label>
                <paravida_components::inputs::NormalInput
                    placeholder="0000"
                    r#type="text"
                    value={jvpm_value}
                    oninput={jvpm_change}
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct DoctorProfilePreviewProps {
    pub profile: paravida_models::ParavidaPractitioner,
}

#[function_component(DoctorProfilePreview)]
fn doctor_profile_preview(props: &DoctorProfilePreviewProps) -> Html {
    let specialty = props
        .profile
        .specialty
        .iter()
        .map(paravida_models::ParavidaSpecialty::display_text)
        .collect::<Vec<_>>()
        .join(", ");

    let email = props
        .profile
        .contacts
        .iter()
        .find(|c| c.system == paravida_models::PractitionerContactSystem::Email)
        .map(|c| c.value.clone())
        .unwrap_or_default();

    let phone = props
        .profile
        .contacts
        .iter()
        .find(|c| c.system == paravida_models::PractitionerContactSystem::Phone)
        .map(|c| c.value.clone())
        .unwrap_or_default();

    let dui = props
        .profile
        .identification
        .iter()
        .find(|i| i.system == paravida_models::IdentifierType::Dui)
        .map(|i| i.value.clone())
        .unwrap_or_default();

    let jvpm = props
        .profile
        .identification
        .iter()
        .find(|i| i.system == paravida_models::IdentifierType::Jvpm)
        .map(|i| i.value.clone())
        .unwrap_or_default();

    html! {
        <paravida_components::Card>
            <div class="flex flex-col gap-4">
                <paravida_components::typography::Highlight>
                    {"Vista Previa del Perfil"}
                </paravida_components::typography::Highlight>

                <div class="space-y-3">
                    <div>
                        <paravida_components::typography::P class="text-sm text-muted">{"Nombre"}</paravida_components::typography::P>
                        <paravida_components::typography::P>
                            {format!("{} {} {}", props.profile.name.suffix, props.profile.name.given, props.profile.name.family)}
                        </paravida_components::typography::P>
                    </div>

                    <div>
                        <paravida_components::typography::P class="text-sm text-muted">{"Especialidad"}</paravida_components::typography::P>
                        <paravida_components::typography::P>{specialty}</paravida_components::typography::P>
                    </div>

                    <div>
                        <paravida_components::typography::P class="text-sm text-muted">{"Email"}</paravida_components::typography::P>
                        <paravida_components::typography::P>{email}</paravida_components::typography::P>
                    </div>

                    <div>
                        <paravida_components::typography::P class="text-sm text-muted">{"Teléfono"}</paravida_components::typography::P>
                        <paravida_components::typography::P>{phone}</paravida_components::typography::P>
                    </div>

                    <div>
                        <paravida_components::typography::P class="text-sm text-muted">{"DUI"}</paravida_components::typography::P>
                        <paravida_components::typography::P>{dui}</paravida_components::typography::P>
                    </div>

                    <div>
                        <paravida_components::typography::P class="text-sm text-muted">{"JVPM"}</paravida_components::typography::P>
                        <paravida_components::typography::P>{jvpm}</paravida_components::typography::P>
                    </div>
                </div>
            </div>
        </paravida_components::Card>
    }
}

pub fn validate_practitioner_profile(
    practitioner: &paravida_models::ParavidaPractitioner,
) -> Vec<String> {
    let mut errors = Vec::<String>::new();
    if practitioner.name.given.is_empty() {
        errors.push("Nombre requerido".to_string());
    }
    if practitioner.name.family.is_empty() {
        errors.push("Apellido requerido".to_string());
    }
    if practitioner.specialty.is_empty() {
        errors.push("Especialidad requerida".to_string());
    }
    if !practitioner
        .contacts
        .iter()
        .any(|contact| contact.system == paravida_models::PractitionerContactSystem::Email)
    {
        errors.push("Email requerido".to_string());
    }
    if !practitioner
        .contacts
        .iter()
        .any(|contact| contact.system == paravida_models::PractitionerContactSystem::Phone)
    {
        errors.push("Telefono requerido".to_string());
    }
    if !practitioner
        .identification
        .iter()
        .any(|identification| identification.system == paravida_models::IdentifierType::Dui)
    {
        errors.push("DUI requerido".to_string());
    }
    if !practitioner
        .identification
        .iter()
        .any(|identification| identification.system == paravida_models::IdentifierType::Jvpm)
    {
        errors.push("JVPM requerido".to_string());
    }

    if let Some(dui) = practitioner
        .identification
        .iter()
        .find(|identification| identification.system == paravida_models::IdentifierType::Dui)
    {
        if dui.value.len() != 10 {
            errors.push(format!(
                "DUI debe tener 10 caracteres, pero tiene {}",
                dui.value.len()
            ));
        }
        if dui
            .value
            .split('-')
            .next()
            .is_none_or(|part| part.len() != 8 || part.parse::<u32>().is_err())
        {
            errors.push("DUI debe tener el formato 123456789-0".to_string());
        }
    }
    if practitioner
        .identification
        .iter()
        .find(|identification| identification.system == paravida_models::IdentifierType::Jvpm)
        .is_none_or(|jvpm| jvpm.value.parse::<u32>().is_err())
    {
        errors.push("JVPM debe tener el formato 0000".to_string());
    }
    errors
}
