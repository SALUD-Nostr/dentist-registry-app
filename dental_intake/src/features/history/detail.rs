use yew::prelude::*;

#[function_component(AppointmentDetailView)]
pub fn appointment_detail_view() -> HtmlResult {
    let appointment = super::hooks::use_appointment()?;

    let Some((id, appointment)) = appointment.as_ref() else {
        return Ok(html! {
            <div class="flex flex-col size-full p-4 items-center justify-center">
                <paravida_components::typography::H2>{"Cita no encontrada"}</paravida_components::typography::H2>
            </div>
        });
    };

    let editable = appointment.status == paravida_models::ParavidaAppointmentStatus::Booked;

    Ok(html! {
        <div class="flex flex-col flex-1 p-4 sm:p-8 gap-3 sm:gap-6">
            <paravida_components::typography::Highlight>
                {"Detalle de Cita"}
            </paravida_components::typography::Highlight>
            <paravida_components::typography::Paragraph class="text-muted mb-2">
                {"Visualiza los detalles de tu cita."}
            </paravida_components::typography::Paragraph>
            <yew::Suspense fallback={html!{
                <div class="flex items-center justify-center size-full">
                    <paravida_components::icons::ParavidaLogo class="size-20 animate-pulse" />
                </div>
            }}>
                <yew::Suspense fallback={html!{
                    <div class="flex items-center justify-center size-full">
                        <paravida_components::icons::ParavidaLogo class="size-20 animate-pulse" />
                    </div>
                }}>
                    <DoctorProfileTag practitioner_pubkey={appointment.practitioner.clone()} />
                </yew::Suspense>
                <div class="flex flex-col md:flex-row gap-3 sm:gap-6 w-full">
                    <AppointmentDetailCard appointment={appointment.clone()} appointment_id={id.clone()} />
                    {editable.then(|| {
                        html! {
                            <div class="flex md:flex-col gap-6 sm:gap-9 w-full md:w-fit mx-auto">
                                <AppointmentCancelModal appointment_id={id.clone()} />
                            </div>
                        }
                    })}
                </div>
            </yew::Suspense>
        </div>
    })
}

#[derive(Properties, PartialEq, Clone)]
struct AppointmentDetailCardProps {
    appointment: paravida_models::ParavidaAppointment,
    appointment_id: String,
}

#[function_component(AppointmentDetailCard)]
fn appointment_detail_card(props: &AppointmentDetailCardProps) -> Html {
    use chrono::Datelike;
    let AppointmentDetailCardProps {
        appointment,
        appointment_id,
    } = props;

    let week_day = match appointment.start.with_timezone(&chrono::Local).weekday() {
        chrono::Weekday::Mon => "Lunes",
        chrono::Weekday::Tue => "Martes",
        chrono::Weekday::Wed => "Miércoles",
        chrono::Weekday::Thu => "Jueves",
        chrono::Weekday::Fri => "Viernes",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    };
    let date = appointment
        .start
        .with_timezone(&chrono::Local)
        .format("%d/%m/%Y")
        .to_string();
    let start = appointment
        .start
        .with_timezone(&chrono::Local)
        .format("%-I:%M %p")
        .to_string();
    let end = appointment
        .end
        .with_timezone(&chrono::Local)
        .format("%-I:%M %p")
        .to_string();
    let specialty = appointment.specialty.display_text();
    let room = appointment.room.name();

    let (status, color) = (
        appointment.status.display_text(),
        match appointment.status {
            paravida_models::ParavidaAppointmentStatus::Booked => classes!("text-secondary"),
            paravida_models::ParavidaAppointmentStatus::Cancelled => classes!("text-red-500"),
        },
    );

    let banner_class = classes!(
        "self-stretch",
        "flex",
        "items-center",
        "justify-center",
        "p-3",
        "sm:p-6",
        "md:p-9",
        "rounded-t-xl",
        match appointment.room {
            paravida_models::ParavidaRoom::General(1) => "bg-room-general",
            paravida_models::ParavidaRoom::UltraSonografia => "bg-room-ultra",
            paravida_models::ParavidaRoom::SaludMental => "bg-room-mental",
            paravida_models::ParavidaRoom::Quirofano => "bg-room-surgical",
            paravida_models::ParavidaRoom::General(4) => "bg-room-general-4",
            _ => "bg-[#000000]",
        },
    );

    let note = appointment
        .note
        .as_ref()
        .filter(|note| !note.is_empty())
        .map(std::string::String::as_str);

    html! {
        <paravida_components::Card class="!p-0">
            <div class="flex flex-col flex-1">
                <div class={banner_class}>
                    <paravida_components::icons::ParavidaLogoBlanco class="size-8 sm:size-12 md:size-16 shrink-0" />
                </div>
                <div class="flex flex-col gap-3 sm:gap-6 flex-1 w-full p-6 md:p-12">
                    <div class="grid grid-cols-2 gap-3 sm:gap-6 items-center">
                        <paravida_components::typography::Highlight>
                            { format!("{room}") }
                        </paravida_components::typography::Highlight>
                        <paravida_components::typography::SubTitle>
                            { specialty }
                        </paravida_components::typography::SubTitle>
                    </div>

                    <div class="flex gap-3 sm:gap-6 items-center">
                        <paravida_components::typography::Paragraph class="font-semibold text-foreground">
                            {"Estado"}
                        </paravida_components::typography::Paragraph>
                        <paravida_components::typography::Paragraph class="font-semibold text-foreground">
                            {"-"}
                        </paravida_components::typography::Paragraph>
                        <paravida_components::typography::Highlight class={color}>
                            { status }
                        </paravida_components::typography::Highlight>
                    </div>

                    <div class="grid grid-cols-2 gap-3 sm:gap-6 items-center">
                        <div class="flex flex-row gap-3 items-center">
                            <paravida_components::icons::Calendar class="size-4 sm:size-6 md:size-8 text-secondary shrink-0" />
                            <div class="flex flex-col">
                                <paravida_components::typography::Paragraph class="font-semibold text-foreground">
                                    {"Fecha"}
                                </paravida_components::typography::Paragraph>
                                <paravida_components::typography::Paragraph>
                                    { format!("{week_day}, {date}") }
                                </paravida_components::typography::Paragraph>
                            </div>
                        </div>

                        <div class="flex flex-row gap-3 items-center">
                            <paravida_components::icons::Clock class="size-4 sm:size-6 md:size-8 text-secondary shrink-0" />
                            <div class="flex flex-col">
                                <paravida_components::typography::Paragraph class="font-semibold text-foreground">
                                    {"Horario"}
                                </paravida_components::typography::Paragraph>
                                <paravida_components::typography::Paragraph>
                                    { format!("{start} - {end}") }
                                </paravida_components::typography::Paragraph>
                            </div>
                        </div>
                    </div>

                    {note.map(|note| {
                        html! {
                            <div class="flex flex-col gap-2">
                                <paravida_components::typography::Paragraph class="font-semibold text-foreground">
                                    {"Notas Adicionales"}
                                </paravida_components::typography::Paragraph>
                                <paravida_components::typography::Paragraph>
                                    { note }
                                </paravida_components::typography::Paragraph>
                            </div>
                        }
                    }).unwrap_or_default()}

                    <div class="flex flex-row gap-2 pt-4 border-t border-muted/30 items-center">
                        <paravida_components::typography::BoldDescription>
                            {"ID de Cita"}
                        </paravida_components::typography::BoldDescription>
                        <paravida_components::typography::BoldDescription>
                            {"-"}
                        </paravida_components::typography::BoldDescription>

                        <paravida_components::typography::Paragraph class="text-muted text-xs break-all">
                            { &appointment_id[..16] }
                        </paravida_components::typography::Paragraph>
                    </div>
                </div>
            </div>
        </paravida_components::Card>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct DoctorProfileTagProps {
    practitioner_pubkey: String,
}

#[function_component(DoctorProfileTag)]
fn doctor_profile_tag(props: &DoctorProfileTagProps) -> HtmlResult {
    let DoctorProfileTagProps {
        practitioner_pubkey,
    } = props;
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let doctor = yew::suspense::use_future_with(
        practitioner_pubkey.clone(),
        |practicer_pubkey| async move {
            let local_db = local_db.as_ref()?;
            let nostr_key = nostr_key.as_ref()?;
            local_db
                .get_doctor(practicer_pubkey.as_str())
                .await
                .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                .ok()
                .and_then(|note| note)
                .and_then(|note| {
                    let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                    let shared_document = mutual_note.view_shared_document(nostr_key).ok()?;
                    paravida_models::ParavidaPractitioner::from_salud_note(&shared_document)
                        .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                        .ok()
                })
        },
    )?;

    let Some(doctor) = doctor.as_ref() else {
        return Ok(html! {
            <paravida_components::Card class="!p-0 !size-fit">
                <div class="flex flex-row size-full select-none">
                    <div class="self-stretch bg-primary p-4 sm:p-6 md:p-8 rounded-l-xl flex justify-center items-center">
                        <paravida_components::icons::ParavidaLogoBlanco class="size-6 sm:size-9 md:size-12 mx-auto shrink-0" />
                    </div>
                    <div class="flex flex-row flex-1 justify-between p-4 sm:p-6 md:p-8 bg-background overflow-clip rounded-r-xl">
                        <div class="flex flex-col flex-1 text-start gap-1 sm:gap-2">
                            <paravida_components::typography::SubTitle>
                                {"Doctor No Encontrado"}
                            </paravida_components::typography::SubTitle>
                            <paravida_components::typography::P class="text-muted text-xs break-all">
                                {practitioner_pubkey.as_str()}
                            </paravida_components::typography::P>
                        </div>
                    </div>
                </div>
            </paravida_components::Card>
        });
    };

    let specialty = doctor
        .specialty
        .first()
        .map(paravida_models::ParavidaSpecialty::display_text);

    let name = if doctor.name.text().trim().is_empty() {
        "Nombre No Definido".to_string()
    } else {
        doctor.name.text()
    };

    Ok(html! {
        <paravida_components::Card class="!p-0">
            <div class="flex flex-row size-full">
                <div class="self-stretch bg-primary p-4 sm:p-6 md:p-8 rounded-l-xl flex justify-center items-center">
                    <paravida_components::icons::ParavidaLogoBlanco class="size-6 sm:size-9 md:size-12 mx-auto shrink-0" />
                </div>
                <div class="flex flex-row flex-1 justify-between p-4 sm:p-6 md:p-8 bg-background overflow-clip rounded-r-xl">
                    <div class="flex flex-col flex-1 text-start gap-1 sm:gap-2">
                        <paravida_components::typography::SubTitle>
                            { name }
                        </paravida_components::typography::SubTitle>
                        <paravida_components::typography::Highlight>
                            { specialty.unwrap_or("Especialidad".to_string()) }
                        </paravida_components::typography::Highlight>
                    </div>
                </div>
            </div>
        </paravida_components::Card>
    })
}

#[derive(Properties, PartialEq, Clone)]
struct AppointmentCancelModalProps {
    appointment_id: String,
}

#[function_component(AppointmentCancelModal)]
fn appointment_cancel_modal(props: &AppointmentCancelModalProps) -> Html {
    let is_open = use_state(|| false);
    let is_confirming = use_state(|| false);
    let confirmed = use_state(|| false);
    let send_server_message = crate::use_send_server_message();

    let open_modal = {
        let is_open = is_open.setter();
        Callback::from(move |()| {
            is_open.set(true);
        })
    };

    let close_modal = {
        let is_open = is_open.setter();
        let is_confirming = is_confirming.setter();
        let confirmed = confirmed.setter();
        Callback::from(move |()| {
            is_open.set(false);
            is_confirming.set(false);
            confirmed.set(false);
        })
    };

    let cancel_appointment = {
        let send_server_message = send_server_message.clone();
        let id = props.appointment_id.clone();
        let is_confirming = is_confirming.setter();
        Callback::from(move |()| {
            let server_message = nostr_minions::nostro2::NostrNote {
                content: id.clone(),
                kind: crate::constants::magic_numbers::ADMIN_APPOINTMENT_CANCEL_REQUEST_KIND,
                ..Default::default()
            };
            send_server_message.emit(server_message);
            is_confirming.set(true);
        })
    };

    let history_ctx = crate::features::nostr_notes::use_history_data();

    // Listen for cancelled appointment confirmation from HistoryStore
    use_effect_with(
        (
            history_ctx.clone(),
            props.appointment_id.clone(),
            is_confirming.clone(),
            confirmed.clone(),
        ),
        move |(history_ctx, id, is_confirming, confirmed)| {
            // Only check if we're in confirming state and haven't confirmed yet
            if !**is_confirming || **confirmed {
                return;
            }

            let Some(history_ctx) = history_ctx.as_ref() else {
                return;
            };

            // Look for the appointment in the store and check if it's cancelled
            for item in &history_ctx.appointments {
                if item.id == *id
                    && item.appointment.status
                        == paravida_models::ParavidaAppointmentStatus::Cancelled
                {
                    confirmed.setter().set(true);
                    return;
                }
            }
        },
    );

    // Auto-close modal after confirmation
    {
        let close_modal_for_effect = close_modal.clone();
        use_effect_with(confirmed.clone(), move |confirmed| {
            if **confirmed {
                let close_modal = close_modal_for_effect.clone();
                yew::platform::spawn_local(async move {
                    yew::platform::time::sleep(std::time::Duration::from_millis(1500)).await;
                    close_modal.emit(());
                });
            }
        });
    }

    html! {
        <>
        <paravida_components::buttons::DangerButton
            class="!w-fit"
            onclick={open_modal.reform(|_| ())}>
            <span class="flex items-center gap-2">
                <paravida_components::icons::CalendarCancel class="size-4 sm:size-6 md:size-8 shrink-0" />
                <span class="">{"Cancelar"}</span>
            </span>
        </paravida_components::buttons::DangerButton>
        <shady_minions::ui::Modal {is_open} >
            <paravida_components::Card class="w-full">
                {if *confirmed {
                    html! {
                        <div class="flex flex-col gap-6 items-center justify-center py-8">
                            <svg
                                class="size-16 text-primary"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M20 6 9 17l-5-5"/>
                            </svg>
                            <paravida_components::typography::Highlight class="text-center">
                                {"Cita Cancelada"}
                            </paravida_components::typography::Highlight>
                            <paravida_components::typography::Paragraph class="text-center text-muted">
                                {"La cita ha sido cancelada exitosamente"}
                            </paravida_components::typography::Paragraph>
                        </div>
                    }
                } else {
                    html! {
                        <div class="flex flex-col gap-3 sm:gap-4 md:gap-6 flex-1 w-full">
                            <paravida_components::typography::SubTitle>
                                {"Cancelar Cita"}
                            </paravida_components::typography::SubTitle>
                            <paravida_components::typography::BoldDescription class="min-w-3xs sm:min-w-xs md:min-lg">
                                {"¿Estás seguro de que quieres cancelar esta cita? Esto no se puede deshacer."}
                            </paravida_components::typography::BoldDescription>
                            <div class="flex flex-row gap-3 sm:gap-4 items-center justify-stretch w-full">
                                <paravida_components::buttons::DangerButton
                                    onclick={cancel_appointment.reform(|_| ())}
                                >
                                    {if *is_confirming {
                                        html! {
                                            <span class="flex items-center gap-2">
                                                <svg
                                                    class="size-4 animate-spin"
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
                                                <span>{"Cancelando..."}</span>
                                            </span>
                                        }
                                    } else {
                                        html! { {"Cancelar Cita"} }
                                    }}
                                </paravida_components::buttons::DangerButton>
                                <paravida_components::buttons::NormalButton
                                    onclick={close_modal.reform(|_| ())}
                                >
                                    {"Volver"}
                                </paravida_components::buttons::NormalButton>
                            </div>
                        </div>
                    }
                }}
            </paravida_components::Card>
        </shady_minions::ui::Modal>
        </>
    }
}
