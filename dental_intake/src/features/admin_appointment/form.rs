use chrono::{Datelike, TimeZone};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdminFormState {
    SelectDoctor,
    SelectRoomAndSpecialty {
        doctor_pubkey: String,
        doctor_name: String,
    },
    CalendarView {
        doctor_pubkey: String,
        doctor_name: String,
        specialty: paravida_models::ParavidaSpecialty,
        room: paravida_models::ParavidaRoom,
    },
    SlotPicked {
        doctor_pubkey: String,
        slot: paravida_models::ParavidaScheduleSlot,
    },
    Confirming {
        doctor_pubkey: String,
        slot_json: String,
    },
}

/// Outer wrapper that conditionally mounts the form only when route is active
#[function_component(AdminAppointmentForm)]
pub fn admin_appointment_form() -> Html {
    let is_active = crate::router::use_is_route_active(crate::router::Route::AdminAppointment);

    if is_active {
        html! { <AdminAppointmentFormInner /> }
    } else {
        // Return empty div to maintain layout
        html! { <div class="size-full" /> }
    }
}

/// Inner component that only mounts when `AdminAppointment` route is active
#[function_component(AdminAppointmentFormInner)]
fn admin_appointment_form_inner() -> Html {
    let form_step = use_state_eq(|| AdminFormState::SelectDoctor);

    // Load all doctors from DB when form mounts
    // use_load_all_doctors_for_form();

    let base_class = classes!(
        "transition-all",
        "duration-300",
        "ease-in-out",
        "transform",
        "size-full",
        "p-4",
        "overflow-hidden",
    );
    let active_class = classes!("translate-x-0", "pointer-events-auto",);
    let inactive_class = classes!("-translate-x-full", "pointer-events-none", "absolute");

    let room_name = match &*form_step {
        AdminFormState::CalendarView { room, .. } => room.name(),
        _ => "Room".to_string(),
    };

    let doctor_name = match &*form_step {
        AdminFormState::SelectRoomAndSpecialty { doctor_name, .. } => doctor_name.clone(),
        AdminFormState::CalendarView { doctor_name, .. } => doctor_name.clone(),
        _ => String::new(),
    };

    html! {
        <div class="overflow-hidden size-full">
            // Step 1: Select Doctor
            <div class={
                classes!(
                    base_class.clone(),
                    if *form_step == AdminFormState::SelectDoctor { active_class.clone() } else { inactive_class.clone() },
                )
            }>
                <div class="size-full flex flex-col">
                    <paravida_components::typography::Highlight>
                        {"Nueva cita administrativa"}
                    </paravida_components::typography::Highlight>
                    <DoctorSelectionStep form_step={form_step.clone()} />
                </div>
            </div>

            // Step 2: Select Specialty & Room
            <div class={
                classes!(
                    base_class.clone(),
                    if matches!(*form_step, AdminFormState::SelectRoomAndSpecialty{..}) { active_class.clone() } else { inactive_class.clone() }
                )
            }>
                <div class="size-full flex flex-col">
                    <div class="flex flex-row w-full justify-between items-center">
                        <paravida_components::typography::Highlight>
                            { format!("Nueva cita - {}", doctor_name) }
                        </paravida_components::typography::Highlight>
                        <BackButton form_step={form_step.clone()} />
                    </div>
                    <RoomAndSpecialtySelectionStep form_step={form_step.clone()} />
                </div>
            </div>

            // Step 3: Calendar View
            <div class={
                classes!(
                    base_class.clone(),
                    if matches!(*form_step, AdminFormState::CalendarView{..}) { active_class.clone() } else { inactive_class.clone() }
                )
            }>
                <div class="size-full flex flex-col">
                    <div class="flex flex-row w-full justify-between items-center">
                        <paravida_components::typography::Highlight>
                            { format!("Disponibilidad - {}", room_name) }
                        </paravida_components::typography::Highlight>
                        <BackButton form_step={form_step.clone()} />
                    </div>
                    <ScheduleCalendarStep form_step={form_step.clone()} />
                </div>
            </div>

            // Step 3: Confirm Appointment
            <div class={
                classes!(
                    base_class.clone(),
                    if matches!(*form_step, AdminFormState::SlotPicked{..}) { active_class.clone() } else { inactive_class.clone() }
                )
            }>
                <div class="size-full flex flex-col">
                    <div class="flex flex-row w-full justify-between items-center max-w-xs sm:max-w-lg md:max-w-xl lg:max-w-3xl">
                        <paravida_components::typography::Highlight>
                            {"Confirmar cita"}
                        </paravida_components::typography::Highlight>
                        <BackButton form_step={form_step.clone()} />
                    </div>
                    <ConfirmationStep form_step={form_step.clone()} />
                </div>
            </div>

            // Step 4: Processing
            <div class={
                classes!(
                    base_class.clone(),
                    if matches!(*form_step, AdminFormState::Confirming{..}) { active_class.clone() } else { inactive_class.clone() }
                )
            }>
                <div class="size-full flex flex-col">
                    <ConfirmingAppointment form_step={form_step.clone()} />
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct FormStepProps {
    pub form_step: UseStateHandle<AdminFormState>,
}

#[function_component(BackButton)]
fn back_button(props: &FormStepProps) -> Html {
    let form_step = props.form_step.clone();
    let onclick = {
        let form_step = form_step.clone();
        Callback::from(move |()| match &*form_step {
            AdminFormState::SelectDoctor => {
                web_sys::console::error_1(&"Cannot go back from SelectDoctor".into());
            }
            AdminFormState::SelectRoomAndSpecialty { .. } => {
                form_step.set(AdminFormState::SelectDoctor);
            }
            AdminFormState::CalendarView {
                doctor_pubkey,
                doctor_name,
                ..
            } => {
                form_step.set(AdminFormState::SelectRoomAndSpecialty {
                    doctor_pubkey: doctor_pubkey.clone(),
                    doctor_name: doctor_name.clone(),
                });
            }
            AdminFormState::SlotPicked {
                doctor_pubkey,
                slot,
            } => {
                // Go back to calendar view, preserving doctor info and selections
                form_step.set(AdminFormState::CalendarView {
                    doctor_pubkey: doctor_pubkey.clone(),
                    doctor_name: "Doctor".to_string(), // TODO: Get from doctor list
                    specialty: slot.specialty,
                    room: slot.room,
                });
            }
            AdminFormState::Confirming { .. } => {
                web_sys::console::error_1(&"Cannot go back from Confirming".into());
            }
        })
    };

    html! {
        <paravida_components::buttons::OutlineButton
            class="!p-2 !w-fit"
            onclick={onclick.reform(|_| ())}>
            <span class="flex items-center gap-2">
                <paravida_components::icons::ChevronLeft class="size-3 sm:size-6 md:size-9" />
                <span class="hidden md:block text-sm sm:text-base">{"Atrás"}</span>
            </span>
        </paravida_components::buttons::OutlineButton>
    }
}

#[function_component(DoctorSelectionStep)]
fn doctor_selection_step(props: &FormStepProps) -> Html {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let selected_doctor = use_state(|| None::<crate::features::nostr_notes::DoctorItem>);
    let errors = use_state(Vec::<String>::new);
    let search_query = use_state(String::new);
    let active_search = use_state(String::new);
    let filtered_doctors = use_state(Vec::<crate::features::nostr_notes::DoctorItem>::new);
    let loading = use_state(|| false);
    let navigator = yew_router::hooks::use_navigator();

    // Load doctors when active_search changes
    use_effect_with(
        (local_db.clone(), nostr_key.clone(), active_search.clone()),
        {
            let filtered_doctors = filtered_doctors.clone();
            let loading = loading.clone();
            move |(local_db, nostr_key, search)| {
                let Some(local_db) = local_db.as_ref() else {
                    return;
                };
                let Some(nostr_key) = nostr_key.as_ref() else {
                    return;
                };

                loading.set(true);
                let local_db = local_db.clone();
                let nostr_key = nostr_key.clone();
                let filtered_doctors = filtered_doctors.clone();
                let loading = loading.clone();
                let search_filter = if search.trim().is_empty() {
                    None
                } else {
                    Some(search.to_string())
                };

                yew::platform::spawn_local(async move {
                    match local_db
                        .get_doctors_paginated_filtered(None, 10, search_filter, &nostr_key)
                        .await
                    {
                        Ok((doctors_notes, _, _)) => {
                            let doctors: Vec<_> = doctors_notes
                                .into_iter()
                                .filter_map(|(pubkey, note)| {
                                    let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                                    let shared_document =
                                        mutual_note.view_shared_document(&nostr_key).ok()?;
                                    let practitioner =
                                        paravida_models::ParavidaPractitioner::from_salud_note(
                                            &shared_document,
                                        )
                                        .ok()?;
                                    Some(crate::features::nostr_notes::DoctorItem {
                                        pubkey,
                                        practitioner,
                                    })
                                })
                                .collect();

                            filtered_doctors.set(doctors);
                            loading.set(false);
                        }
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Error loading filtered doctors: {e:#?}").into(),
                            );
                            loading.set(false);
                        }
                    }
                });
            }
        },
    );

    let on_select_doctor = {
        let selected_doctor = selected_doctor.clone();
        let errors = errors.setter();
        Callback::from(move |doctor: crate::features::doctors::DoctorItem| {
            selected_doctor.set(Some(doctor));
            errors.set(Vec::new());
        })
    };

    let on_continue = {
        let selected_doctor = selected_doctor.clone();
        let form_step = props.form_step.setter();
        let errors = errors.setter();
        Callback::from(move |_| {
            let Some(doctor) = (*selected_doctor).clone() else {
                errors.set(vec!["Por favor seleccione un doctor".to_string()]);
                return;
            };

            form_step.set(AdminFormState::SelectRoomAndSpecialty {
                doctor_pubkey: doctor.pubkey.clone(),
                doctor_name: doctor.practitioner.name.text(),
            });
        })
    };

    let on_search_input = {
        let search_query = search_query.setter();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let do_search = {
        let search_query = search_query.clone();
        let active_search = active_search.clone();
        move || {
            active_search.set((*search_query).clone());
        }
    };

    let on_search_click = {
        let do_search = do_search.clone();
        Callback::from(move |_: MouseEvent| {
            do_search();
        })
    };

    let on_search_enter = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                do_search();
            }
        })
    };

    let on_clear_search = {
        let search_query = search_query.clone();
        let active_search = active_search.clone();
        Callback::from(move |_: MouseEvent| {
            search_query.set(String::new());
            active_search.set(String::new());
        })
    };

    let on_create_doctor_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Some(navigator) = navigator.as_ref() {
                navigator.push(&crate::router::AppRoute::CreateDoctor);
            }
        })
    };

    let input_class = classes!(
        "col-span-2",
        "flex-1",
        "border",
        "border-muted",
        "text-foreground",
        "text-sm",
        "sm:text-base",
        "rounded-lg",
        "py-3",
        "px-4",
        "shadow-sm",
        "border-primary",
        "focus:outline-none",
        "focus:border-secondary",
        "transition-all",
        "placeholder:text-muted",
        "duration-150",
    );

    html! {
        <div class="flex flex-col size-full overflow-hidden">
            // Search and Button Row
            <div class="flex flex-wrap gap-2 items-center my-2">
                <input
                    type="text"
                    class={input_class.clone()}
                    placeholder="Buscar por nombre o especialidad..."
                    value={(*search_query).clone()}
                    oninput={on_search_input}
                    onkeypress={on_search_enter}
                />
                <button
                    onclick={on_search_click}
                    class={classes!(
                        "px-4",
                        "py-3",
                        "bg-primary",
                        "text-white",
                        "rounded-lg",
                        "hover:bg-primary/90",
                        "transition-colors",
                        "text-sm",
                        "font-medium",
                        "whitespace-nowrap",
                    )}
                >
                    {"Buscar"}
                </button>
                if !active_search.is_empty() {
                    <button
                        onclick={on_clear_search}
                        class={classes!(
                            "px-4",
                            "py-3",
                            "border",
                            "border-muted",
                            "text-foreground",
                            "rounded-lg",
                            "hover:bg-muted/10",
                            "transition-colors",
                            "text-sm",
                            "font-medium",
                            "whitespace-nowrap",
                        )}
                    >
                        {"Limpiar"}
                    </button>
                }
                <paravida_components::buttons::OutlineButton
                    onclick={on_create_doctor_click}
                    class="!w-auto !px-6"
                >
                    {"Nuevo Doctor"}
                </paravida_components::buttons::OutlineButton>
                <paravida_components::buttons::NormalButton
                    onclick={on_continue}
                    class="!w-auto !px-6"
                >
                    {"Continuar"}
                </paravida_components::buttons::NormalButton>
            </div>

            if *loading {
                <paravida_components::Card class="!max-w-2xl">
                    <div class="flex flex-col items-center justify-center gap-4 py-8">
                        <paravida_components::icons::ParavidaLogo class="size-12 animate-pulse" />
                        <paravida_components::typography::P class="text-muted text-center">
                            {"Cargando doctores..."}
                        </paravida_components::typography::P>
                    </div>
                </paravida_components::Card>
            } else if filtered_doctors.is_empty() && !active_search.is_empty() {
                <paravida_components::Card class="!max-w-2xl">
                    <div class="flex flex-col items-center justify-center gap-4 py-8">
                        <paravida_components::typography::P class="text-muted text-center">
                            {"No se encontraron doctores que coincidan con la búsqueda."}
                        </paravida_components::typography::P>
                    </div>
                </paravida_components::Card>
            } else if filtered_doctors.is_empty() {
                <paravida_components::Card class="!max-w-2xl">
                    <div class="flex flex-col items-center justify-center gap-4 py-8">
                        <paravida_components::typography::P class="text-muted text-center">
                            {"No hay doctores disponibles en el sistema."}
                        </paravida_components::typography::P>
                        <paravida_components::typography::P class="text-muted text-sm text-center">
                            {"Los doctores se cargarán automáticamente cuando se sincronicen sus perfiles."}
                        </paravida_components::typography::P>
                    </div>
                </paravida_components::Card>
            } else {
                // Doctors Table
                <div class="flex-1">
                    <div class="bg-white rounded-lg shadow-md border border-muted/30 overflow-hidden">
                        // Header
                        <div class="grid grid-cols-12 gap-4 bg-primary/10 px-6 py-3 font-semibold text-sm text-foreground border-b border-muted/30">
                            <div class="col-span-6">{"Nombre"}</div>
                            <div class="col-span-5">{"Especialidades"}</div>
                            <div class="col-span-1" />
                        </div>

                        // Rows
                        <div class="divide-y divide-muted/20 max-h-[calc(84vh-12rem)] overflow-y-auto">
                            { for filtered_doctors.iter().map(|doctor| {
                                let doctor_clone = doctor.clone();
                                let is_selected = selected_doctor.as_ref()
                                    .is_some_and(|d| d.pubkey == doctor.pubkey);
                                let onclick = on_select_doctor.reform(move |_| doctor_clone.clone());

                                let specialties = doctor
                                    .practitioner
                                    .specialty
                                    .iter()
                                    .map(paravida_models::ParavidaSpecialty::display_text)
                                    .collect::<Vec<_>>()
                                    .join(", ");

                                let row_class = if is_selected {
                                    "grid grid-cols-12 gap-4 px-6 py-4 hover:bg-primary/5 transition-colors cursor-pointer bg-primary/10 border-l-4 border-primary"
                                } else {
                                    "grid grid-cols-12 gap-4 px-6 py-4 hover:bg-primary/5 transition-colors cursor-pointer"
                                };

                                html! {
                                    <div
                                        key={doctor.pubkey.clone()}
                                        class={row_class}
                                        onclick={onclick}
                                    >
                                        <div class="col-span-6 flex items-center">
                                            <span class="text-sm font-medium text-foreground">
                                                {doctor.practitioner.name.text()}
                                            </span>
                                        </div>
                                        <div class="col-span-5 flex items-center">
                                            <span class="text-sm text-muted">
                                                {specialties}
                                            </span>
                                        </div>
                                        <div class="col-span-1 flex items-center justify-end">
                                            if is_selected {
                                                <paravida_components::icons::Check class="size-5 text-primary" />
                                            }
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>
            }

            <paravida_components::alerts::FormErrors errors={(*errors).clone()} />
        </div>
    }
}

#[function_component(RoomAndSpecialtySelectionStep)]
fn room_and_specialty_selection_step(props: &FormStepProps) -> Html {
    let (doctor_pubkey, doctor_name) = match &*props.form_step {
        AdminFormState::SelectRoomAndSpecialty {
            doctor_pubkey,
            doctor_name,
        } => (doctor_pubkey.clone(), doctor_name.clone()),
        _ => (String::new(), String::new()),
    };

    let specialty_ref = use_mut_ref(|| None::<paravida_models::ParavidaSpecialty>);
    let room_ref = use_mut_ref(|| None::<paravida_models::ParavidaRoom>);
    let errors = use_state(Vec::<String>::new);

    let submit_step = {
        let specialty_ref = specialty_ref.clone();
        let room_ref = room_ref.clone();
        let form_step = props.form_step.setter();
        let errors = errors.setter();
        let doctor_pubkey = doctor_pubkey.clone();
        let doctor_name = doctor_name.clone();
        Callback::from(move |_| {
            let mut new_errors = Vec::<String>::new();

            let Some(specialty) = *specialty_ref.borrow() else {
                new_errors.push("Por favor seleccione una especialidad".to_string());
                errors.set(new_errors);
                return;
            };

            let Some(room) = *room_ref.borrow() else {
                new_errors.push("Por favor seleccione una sala".to_string());
                errors.set(new_errors);
                return;
            };

            if new_errors.is_empty() {
                form_step.set(AdminFormState::CalendarView {
                    doctor_pubkey: doctor_pubkey.clone(),
                    doctor_name: doctor_name.clone(),
                    specialty,
                    room,
                });
            }
        })
    };

    html! {
        <div class="flex flex-col gap-4 my-4">
            <paravida_components::typography::Paragraph>
                {"Selecciona la especialidad y sala para ver los horarios disponibles."}
            </paravida_components::typography::Paragraph>

            <SpecialtySelect specialty={specialty_ref.clone()} errors={errors.clone()} />
            <RoomSelect room={room_ref.clone()} errors={errors.clone()} />

            <paravida_components::buttons::NormalButton
                onclick={submit_step}
                class="!w-auto"
            >
                {"Ver Disponibilidad"}
            </paravida_components::buttons::NormalButton>

            <paravida_components::alerts::FormErrors errors={(*errors).clone()} />
        </div>
    }
}

#[function_component(ScheduleCalendarStep)]
fn schedule_calendar_step(props: &FormStepProps) -> Html {
    let (specialty, room, doctor_pubkey) = match &*props.form_step {
        AdminFormState::CalendarView {
            specialty,
            room,
            doctor_pubkey,
            ..
        } => (*specialty, *room, doctor_pubkey.clone()),
        _ => return html! {},
    };

    html! {
        <ScheduleCalendar
            {specialty}
            {room}
            {doctor_pubkey}
            form_step={props.form_step.clone()}
        />
    }
}

#[derive(Properties, Clone)]
struct RoomSelectProps {
    pub room: std::rc::Rc<std::cell::RefCell<Option<paravida_models::ParavidaRoom>>>,
    pub errors: UseStateHandle<Vec<String>>,
    #[prop_or_default]
    pub on_change: Option<Callback<()>>,
}

impl PartialEq for RoomSelectProps {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.room, &other.room) && self.errors == other.errors
    }
}

#[function_component(RoomSelect)]
fn room_select(props: &RoomSelectProps) -> Html {
    let onchange = {
        let room_ref = props.room.clone();
        let errors = props.errors.setter();
        let on_change_cb = props.on_change.clone();
        Callback::from(move |room: Option<paravida_models::ParavidaRoom>| {
            if let Some(room) = room {
                room_ref.borrow_mut().replace(room);
                errors.set(Vec::<String>::new());
                if let Some(ref callback) = on_change_cb {
                    callback.emit(());
                }
            }
        })
    };
    html! {
        <shady_minions::ui::Select<paravida_models::ParavidaRoom> {onchange} class="flex-1">
            <shady_minions::ui::SelectTrigger<paravida_models::ParavidaRoom> label="Escoger sala" />
            <shady_minions::ui::SelectContent<paravida_models::ParavidaRoom>>
                { for paravida_models::ParavidaRoom::all().iter().map(|room| {
                    html! {
                        <shady_minions::ui::SelectItem<paravida_models::ParavidaRoom> value={*room} label={room.name()} />
                    }
                })}
            </shady_minions::ui::SelectContent<paravida_models::ParavidaRoom>>
        </shady_minions::ui::Select<paravida_models::ParavidaRoom>>
    }
}

#[derive(Properties, Clone)]
struct SpecialtySelectProps {
    pub specialty: std::rc::Rc<std::cell::RefCell<Option<paravida_models::ParavidaSpecialty>>>,
    pub errors: UseStateHandle<Vec<String>>,
    #[prop_or_default]
    pub on_change: Option<Callback<()>>,
}

impl PartialEq for SpecialtySelectProps {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.specialty, &other.specialty) && self.errors == other.errors
    }
}

#[function_component(SpecialtySelect)]
fn specialty_select(props: &SpecialtySelectProps) -> Html {
    let onchange = {
        let specialty_ref = props.specialty.clone();
        let errors = props.errors.setter();
        let on_change_cb = props.on_change.clone();
        Callback::from(
            move |specialty: Option<paravida_models::ParavidaSpecialty>| {
                if let Some(specialty) = specialty {
                    specialty_ref.borrow_mut().replace(specialty);
                    errors.set(Vec::<String>::new());
                    if let Some(ref callback) = on_change_cb {
                        callback.emit(());
                    }
                }
            },
        )
    };
    html! {
        <shady_minions::ui::Select<paravida_models::ParavidaSpecialty> {onchange} class="flex-1">
            <shady_minions::ui::SelectTrigger<paravida_models::ParavidaSpecialty> label="Escoger especialidad" />
            <shady_minions::ui::SelectContent<paravida_models::ParavidaSpecialty>>
                { for paravida_models::ParavidaSpecialty::all().iter().map(|specialty| {
                    html! {
                        <shady_minions::ui::SelectItem<paravida_models::ParavidaSpecialty> value={*specialty} label={specialty.display_text()} />
                    }
                })}
            </shady_minions::ui::SelectContent<paravida_models::ParavidaSpecialty>>
        </shady_minions::ui::Select<paravida_models::ParavidaSpecialty>>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ScheduleCalendarProps {
    pub specialty: paravida_models::ParavidaSpecialty,
    pub room: paravida_models::ParavidaRoom,
    pub doctor_pubkey: String,
    pub form_step: UseStateHandle<AdminFormState>,
}

#[function_component(ScheduleCalendar)]
fn schedule_calendar(props: &ScheduleCalendarProps) -> Html {
    // ALL HOOKS MUST BE CALLED BEFORE ANY CONDITIONAL RETURNS
    let schedule = crate::features::room_schedules::use_room_schedule();
    let calendar_state = use_mut_ref(|| None::<yew_full_calendar::Calendar>);
    let errors = use_state(Vec::<String>::new);
    let relay_ctx = nostr_minions::use_nostr_relay_pool();
    let slot_date = use_state(chrono::Local::now);
    let start_slot = use_state(|| {
        ThirtyMinuteSlot::from_local_time(chrono::Local::now() + chrono::Duration::hours(1))
    });
    let end_slot = use_state(|| {
        ThirtyMinuteSlot::from_local_time(chrono::Local::now() + chrono::Duration::hours(2))
    });

    // Get room_schedule as Option - don't unwrap yet
    let room_schedule = schedule.find_schedule(props.room).cloned();

    // Call use_effect_with with Option
    use_effect_with((relay_ctx.last_note.clone(), room_schedule.clone()), {
        let calendar_state = calendar_state.clone();
        move |(last_note, schedule_opt)| {
            let Some(nostr_minions::nostro2::NostrRelayEvent::NewNote(.., _note_sub_id, last_note)) =
                last_note
            else {
                return;
            };
            if last_note.pubkey != crate::PARAVIDA_PUBKEY
                || last_note.kind != crate::constants::magic_numbers::SCHEDULE_NOTE_KIND
            {
                return;
            }

            let Some(schedule) = schedule_opt else {
                return;
            };

            if let Some(cal) = calendar_state.borrow().as_ref() {
                let calendar_clone = cal.clone();
                let schedule = schedule.slots.clone();
                let fun = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                    calendar_clone.clear_events();
                    for slot in &schedule {
                        let start_iso = slot.start_iso_string();
                        let end_iso = slot.end_iso_string();
                        let new_event = yew_full_calendar::EventBuilder::default()
                            .display(yew_full_calendar::EventDisplay::Background)
                            .background_color("#ff0000")
                            .start(yew_full_calendar::EventDate::Iso8601String(start_iso))
                            .end(yew_full_calendar::EventDate::Iso8601String(end_iso));
                        let _ = calendar_clone.add_or_replace_event(new_event);
                    }
                })
                .into();
                cal.batch_rendering(fun);
            }
        }
    });

    // NOW we can do conditional logic
    let Some(room_schedule) = room_schedule else {
        return html! {
            <paravida_components::typography::P class="text-muted">
                {"No hay horarios disponibles para esta sala"}
            </paravida_components::typography::P>
        };
    };

    let set_calendar = {
        let calendar_state = calendar_state.clone();
        let schedule = room_schedule.clone();
        Callback::from(move |cal: yew_full_calendar::Calendar| {
            let calendar_clone = cal.clone();
            let schedule = schedule.slots.clone();
            let fun = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                for slot in &schedule {
                    let start_iso = slot.start_iso_string();
                    let end_iso = slot.end_iso_string();
                    let new_event = yew_full_calendar::EventBuilder::default()
                        .display(yew_full_calendar::EventDisplay::Background)
                        .background_color("#ff0000")
                        .start(yew_full_calendar::EventDate::Iso8601String(start_iso))
                        .end(yew_full_calendar::EventDate::Iso8601String(end_iso));

                    let _ = calendar_clone.add_or_replace_event(new_event);
                }
            })
            .into();
            cal.batch_rendering(fun);
            calendar_state.borrow_mut().replace(cal);
        })
    };

    fn overlaps(
        a_start: chrono::DateTime<chrono::Utc>,
        a_end: chrono::DateTime<chrono::Utc>,
        b_start: chrono::DateTime<chrono::Utc>,
        b_end: chrono::DateTime<chrono::Utc>,
    ) -> bool {
        a_start < b_end && b_start < a_end
    }

    let submit_slot = {
        let slot_start = start_slot.clone();
        let slot_end = end_slot.clone();
        let room = props.room;
        let specialty = props.specialty;
        let errors = errors.setter();
        let schedule = room_schedule.clone();
        let form_step = props.form_step.setter();
        let slot_date = slot_date.clone();
        let doctor_pubkey = props.doctor_pubkey.clone();
        Callback::from(move |()| {
            use chrono::Timelike;
            let mut new_errors = Vec::<String>::new();
            let (start_hour, start_minute) = slot_start.hours_minutes();
            let (end_hour, end_minute) = slot_end.hours_minutes();
            let Some(start) = (*slot_date)
                .with_hour(start_hour)
                .and_then(|dt| dt.with_minute(start_minute))
                .map(|dt| dt.to_utc())
            else {
                new_errors.push("La hora de inicio no es válida".to_string());
                errors.set(new_errors);
                return;
            };
            let Some(end) = (*slot_date)
                .with_hour(end_hour)
                .and_then(|dt| dt.with_minute(end_minute))
                .map(|dt| dt.to_utc())
            else {
                new_errors.push("La hora de fin no es válida".to_string());
                errors.set(new_errors);
                return;
            };

            if schedule
                .slots
                .iter()
                .any(|ev| overlaps(start, end, ev.start(), ev.end()))
            {
                new_errors.push("Horario ya reservado".to_string());
                errors.set(new_errors);
                return;
            }

            match paravida_models::ParavidaScheduleSlot::new_slot(room, start, end, specialty) {
                Ok(new_slot) => {
                    form_step.set(AdminFormState::SlotPicked {
                        doctor_pubkey: doctor_pubkey.clone(),
                        slot: new_slot,
                    });
                }
                Err(e) => match e {
                    paravida_models::SlotError::StartAfterEnd => {
                        new_errors.push(
                            "La hora de inicio no puede ser posterior a la de fin".to_string(),
                        );
                    }
                    paravida_models::SlotError::MustBeSameDate => {
                        new_errors.push(
                            "La hora de inicio y la de fin deben ser del mismo día".to_string(),
                        );
                    }
                    paravida_models::SlotError::AlreadyStarted => {
                        if start > chrono::Utc::now() - chrono::Duration::hours(1) {
                            let unchecked_start =
                                paravida_models::ParavidaScheduleSlot::slot_unchecked(
                                    room, start, end, specialty, None,
                                );
                            form_step.set(AdminFormState::SlotPicked {
                                doctor_pubkey: doctor_pubkey.clone(),
                                slot: unchecked_start,
                            });
                            return;
                        }
                        new_errors.push("La hora de inicio no puede ser en el pasado.".to_string());
                    }
                    _ => {
                        new_errors.push("La fecha no es válida".to_string());
                    }
                },
            }
            errors.set(new_errors);
        })
    };

    let initial_date = chrono::Local::now().to_rfc3339();
    let first_day = {
        use chrono::Datelike;
        chrono::Local::now().weekday().number_from_monday()
    };

    let on_date_select = {
        let start_date = start_slot.setter();
        let end_date = end_slot.setter();
        let errors = errors.setter();
        let slot_date = slot_date.clone();
        let schedule = room_schedule.clone();
        Callback::from(move |date: yew_full_calendar::SelectionInfo| {
            let mut new_errors = Vec::<String>::new();
            let Some(utc_start) = date.start_str().and_then(|s| {
                s.parse::<chrono::DateTime<chrono::Local>>()
                    .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                    .ok()
            }) else {
                new_errors.push("La hora de inicio no es válida".to_string());
                errors.set(new_errors);
                return;
            };
            let Some(utc_end) = date.end_str().and_then(|s| {
                s.parse::<chrono::DateTime<chrono::Local>>()
                    .inspect_err(|e| web_sys::console::error_1(&format!("{e:#?}").into()))
                    .ok()
            }) else {
                new_errors.push("La hora de fin no es válida".to_string());
                errors.set(new_errors);
                return;
            };
            if schedule
                .slots
                .iter()
                .any(|ev| overlaps(utc_start.into(), utc_end.into(), ev.start(), ev.end()))
            {
                new_errors.push("Horario ya reservado".to_string());
                errors.set(new_errors);
                return;
            }
            start_date.set(ThirtyMinuteSlot::from_local_time(utc_start));
            end_date.set(ThirtyMinuteSlot::from_local_time(utc_end));
            slot_date.set(utc_start);
            errors.set(new_errors);
        })
    };

    let change_date = {
        let calendar_state = calendar_state.clone();
        let start_slot = start_slot.clone();
        let date_slot = slot_date.setter();
        let errors = errors.setter();
        Callback::from(move |date_input: InputEvent| {
            let mut new_errors = Vec::<String>::new();
            let Some(date) = date_input.target_dyn_into::<web_sys::HtmlInputElement>() else {
                return;
            };
            let js_date =
                web_sys::js_sys::Date::new(&web_sys::js_sys::Date::parse(&date.value()).into());

            let Some(chrono_date) = chrono::NaiveDate::parse_from_str(&date.value(), "%Y-%m-%d")
                .map(|date| date.and_time(start_slot.naive_time()))
                .inspect_err(|e| {
                    web_sys::console::error_1(&format!("{e:#?}").into());
                })
                .ok()
                .and_then(|naive_dt| chrono::Local.from_local_datetime(&naive_dt).single())
            else {
                new_errors.push("La fecha no es válida".to_string());
                errors.set(new_errors);
                return;
            };

            if let Some(cal) = calendar_state.borrow().as_ref() {
                cal.update_view_to_date(&yew_full_calendar::InitialView::TimeGridWeek, &js_date);
                start_slot.set(ThirtyMinuteSlot::from_local_time(chrono_date));
                date_slot.set(chrono_date);
            }
            errors.set(new_errors);
        })
    };

    let calendar_options = yew_full_calendar::Options::default()
        .with_slot_label_format(yew_full_calendar::EventTimeFormat {
            hour: yew_full_calendar::DateFormat::TwoDigit,
            minute: yew_full_calendar::DateFormat::TwoDigit,
            meridiem: yew_full_calendar::DateFormat::Short,
            omit_zero_minute: Some(true),
        })
        .with_initial_date(yew_full_calendar::EventDate::Iso8601String(initial_date))
        .with_first_day(first_day)
        .with_initial_view(yew_full_calendar::InitialView::TimeGridWeek)
        .with_slot_duration(yew_full_calendar::EventDuration::DurationObject(
            yew_full_calendar::EventDurationObject {
                years: 0,
                months: 0,
                days: 0,
                milliseconds: 60 * 30 * 1000,
            },
        ))
        .with_locale(yew_full_calendar::Locale::Es)
        .with_header_toolbar(yew_full_calendar::HeaderOptions::Options(
            yew_full_calendar::HeaderSections {
                left: String::new(),
                center: String::new(),
                right: String::new(),
            },
        ))
        .with_event_time_format(yew_full_calendar::EventTimeFormat {
            hour: yew_full_calendar::DateFormat::Short,
            minute: yew_full_calendar::DateFormat::Short,
            meridiem: yew_full_calendar::DateFormat::Short,
            omit_zero_minute: Some(true),
        })
        .with_all_day_slot(false)
        .with_selectable(true);

    let input_class = classes!(
        "w-full",
        "sm:max-w-lg",
        "md:max-w-xl",
        "lg:max-w-3xl",
        "flex-1",
        "border",
        "border-muted",
        "text-foreground",
        "text-sm",
        "sm:text-base",
        "md:text-lg",
        "rounded-lg",
        "py-3",
        "px-4",
        "shadow-sm",
        "border-primary",
        "focus:outline-none",
        "focus:border-secondary",
        "transition-all",
        "placeholder:text-sm",
        "placeholder:sm:text-base",
        "placeholder:md:text-lg",
        "placeholder:text-muted",
        "duration-150",
    );

    let date_value = slot_date.format("%Y-%m-%d").to_string();

    html! {
        <>
            <div class="hidden sm:flex flex-row gap-2 items-center my-2">
                <input
                    class={input_class.clone()}
                    type="date"
                    value={date_value.clone()}
                    oninput={change_date.clone()}
                />
                <ThirtyMinuteSlotPicker slot={start_slot.clone()} />
                <ThirtyMinuteSlotPicker slot={end_slot.clone()} />
                <paravida_components::buttons::NormalButton
                    onclick={submit_slot.reform(|_| ())}
                >
                    {"Continuar"}
                </paravida_components::buttons::NormalButton>
            </div>

            <div class="block sm:hidden grid grid-cols-2 gap-2 items-center place-items-center my-2">
                <input
                    class={input_class.clone()}
                    type="date"
                    value={date_value}
                    oninput={change_date}
                />
                <paravida_components::buttons::NormalButton
                    class="!size-full"
                    onclick={submit_slot.reform(|_| ())}
                >
                    {"Confirmar"}
                </paravida_components::buttons::NormalButton>
                <ThirtyMinuteSlotPicker slot={start_slot.clone()} />
                <ThirtyMinuteSlotPicker slot={end_slot.clone()} />
            </div>

            <paravida_components::Card class="!p-0 !max-w-none mb-2">
                <style>
                    {r"
                    .fc-header-toolbar {
                        margin-bottom: 0px !important;
                    }
                    "}
                </style>
                <yew_full_calendar::FullCalendarComponent
                    class="size-full flex-1 lg:max-h-[calc(74vh-4rem)] !rounded-xl"
                    calendar_id="admin-calendar"
                    {calendar_options}
                    {on_date_select}
                    on_calendar_created={set_calendar}
                />
            </paravida_components::Card>

            <paravida_components::alerts::FormErrors errors={(*errors).clone()} />
        </>
    }
}

#[function_component(ConfirmationStep)]
fn confirmation_step(props: &FormStepProps) -> Html {
    let notes = use_state(String::new);
    let errors = use_state(Vec::<String>::new);
    let send_server_message = crate::use_send_server_message();

    let on_notes_change = {
        let notes = notes.setter();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            notes.set(input.value());
        })
    };

    let on_confirm = {
        let notes = notes.clone();
        let form_step = props.form_step.clone();
        let errors = errors.setter();
        let send_server_message = send_server_message.clone();
        Callback::from(move |_| {
            let AdminFormState::SlotPicked {
                doctor_pubkey,
                mut slot,
            } = (*form_step).clone()
            else {
                return;
            };

            // Add notes to the slot
            if !notes.is_empty() {
                slot.set_note((*notes).clone());
            }

            // Send server message with slot data
            let mut server_message = nostr_minions::nostro2::NostrNote {
                content: serde_json::to_string(&slot).unwrap_or_default(),
                kind: crate::constants::magic_numbers::ADMIN_APPOINTMENT_REQUEST_KIND,
                ..Default::default()
            };

            // Add doctor pubkey as a tag
            server_message.tags.add_parameter_tag(&doctor_pubkey);

            send_server_message.emit(server_message);

            // Transition to Confirming state with slot data
            let slot_json = serde_json::to_string(&slot).unwrap_or_default();
            form_step.set(AdminFormState::Confirming {
                doctor_pubkey,
                slot_json,
            });
            errors.set(Vec::new());
        })
    };

    let AdminFormState::SlotPicked {
        // doctor_pubkey,
        slot,
        ..
    } = &*props.form_step
    else {
        return html! {};
    };

    html! {
        <div class="flex flex-col gap-3 sm:gap-6">
            <paravida_components::typography::Paragraph class="my-2">
                {"Revisa los detalles de la cita y confirma la reserva."}
            </paravida_components::typography::Paragraph>

            <AppointmentPreview slot={slot.clone()} />

            <div class="flex flex-col gap-1 sm:gap-3 w-full sm:max-w-lg md:max-w-xl lg:max-w-3xl">
                <label class="text-foreground font-medium">
                    {"Notas adicionales (opcional)"}
                </label>
                <textarea
                    class={classes!(
                        "w-full",
                        "border",
                        "border-muted",
                        "text-foreground",
                        "text-sm",
                        "sm:text-base",
                        "rounded-lg",
                        "py-3",
                        "px-4",
                        "shadow-sm",
                        "border-primary",
                        "focus:outline-none",
                        "focus:border-secondary",
                        "transition-all",
                        "placeholder:text-muted",
                        "duration-150",
                        "min-h-16",
                    )}
                    placeholder="Agregar notas sobre la cita..."
                    value={(*notes).clone()}
                    oninput={on_notes_change}
                />
            </div>

            <div class="flex flex-row gap-4">
                <paravida_components::buttons::NormalButton
                    onclick={on_confirm}
                >
                    {"Confirmar cita"}
                </paravida_components::buttons::NormalButton>
            </div>

            <paravida_components::alerts::FormErrors errors={(*errors).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct AppointmentPreviewProps {
    pub slot: paravida_models::ParavidaScheduleSlot,
}

#[function_component(AppointmentPreview)]
fn appointment_preview(props: &AppointmentPreviewProps) -> Html {
    let specialty = props.slot.specialty.display_text();
    let room = props.slot.room.name();
    let room_banner_class = classes!(
        "flex-1",
        "flex",
        "justify-center",
        "items-center",
        "p-4",
        "w-full",
        "sm:w-fit",
        "rounded-t-xl",
        "sm:rounded-l-xl",
        "sm:rounded-tr-none",
        match props.slot.room {
            paravida_models::ParavidaRoom::General(1) => "bg-room-general",
            paravida_models::ParavidaRoom::UltraSonografia => "bg-room-ultra",
            paravida_models::ParavidaRoom::SaludMental => "bg-room-mental",
            paravida_models::ParavidaRoom::Quirofano => "bg-room-surgical",
            paravida_models::ParavidaRoom::General(4) => "bg-room-general-4",
            _ => "bg-[#000000]",
        },
    );
    let week_day = match props.slot.start().with_timezone(&chrono::Local).weekday() {
        chrono::Weekday::Mon => "Lunes",
        chrono::Weekday::Tue => "Martes",
        chrono::Weekday::Wed => "Miércoles",
        chrono::Weekday::Thu => "Jueves",
        chrono::Weekday::Fri => "Viernes",
        chrono::Weekday::Sat => "Sábado",
        chrono::Weekday::Sun => "Domingo",
    };
    let date = props
        .slot
        .start()
        .with_timezone(&chrono::Local)
        .format("%Y-%m-%d")
        .to_string();
    let start = props
        .slot
        .start()
        .with_timezone(&chrono::Local)
        .format("%H:%M")
        .to_string();
    let end = props
        .slot
        .end()
        .with_timezone(&chrono::Local)
        .format("%H:%M")
        .to_string();

    html! {
        <paravida_components::Card class="!p-0">
            <div class="flex flex-col sm:flex-row min-h-64 flex-1 w-full">
                <div
                    class={room_banner_class}>
                    <paravida_components::icons::ParavidaLogoBlanco class="size-6 sm:size-9 md:size-12 mx-auto shrink-0" />
                </div>
                <div class="flex-5 p-6 gap-1 sm:gap-3 flex flex-col text-start items-start w-full">
                    <paravida_components::typography::SubTitle class="text-nowrap">
                        { room }
                    </paravida_components::typography::SubTitle>
                    <paravida_components::typography::Highlight>
                        { specialty }
                    </paravida_components::typography::Highlight>
                    <div class="grid grid-cols-2 gap-3 sm:gap-6 mt-3 shrink-0 size-fit w-full">
                        <div class="flex flex-row gap-3 items-center col-span-2">
                            <paravida_components::icons::Calendar class="size-6 text-secondary shrink-0" />
                            <paravida_components::typography::SubTitle class="text-start">
                                { format!("{week_day}, {date}") }
                            </paravida_components::typography::SubTitle>
                        </div>
                        <div class="flex flex-row gap-3 items-center col-span-2">
                            <paravida_components::icons::Clock class="size-6 text-secondary shrink-0" />
                            <paravida_components::typography::SubTitle class="text-start">
                                { format!("{start} - {end}") }
                            </paravida_components::typography::SubTitle>
                        </div>
                    </div>
                </div>
            </div>
        </paravida_components::Card>
    }
}

// ThirtyMinuteSlot enum and helpers (copied from doctors_new)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThirtyMinuteSlot {
    TwelveAm,
    TwelveThirtyAm,
    OneAm,
    OneThirtyAm,
    TwoAm,
    TwoThirtyAm,
    ThreeAm,
    ThreeThirtyAm,
    FourAm,
    FourThirtyAm,
    FiveAm,
    FiveThirtyAm,
    SixAm,
    SixThirtyAm,
    SevenAm,
    SevenThirtyAm,
    EightAm,
    EightThirtyAm,
    NineAm,
    NineThirtyAm,
    TenAm,
    TenThirtyAm,
    ElevenAm,
    ElevenThirtyAm,
    TwelvePm,
    TwelveThirtyPm,
    OnePm,
    OneThirtyPm,
    TwoPm,
    TwoThirtyPm,
    ThreePm,
    ThreeThirtyPm,
    FourPm,
    FourThirtyPm,
    FivePm,
    FiveThirtyPm,
    SixPm,
    SixThirtyPm,
    SevenPm,
    SevenThirtyPm,
    EightPm,
    EightThirtyPm,
    NinePm,
    NineThirtyPm,
    TenPm,
    TenThirtyPm,
    ElevenPm,
    ElevenThirtyPm,
}

impl AsRef<str> for ThirtyMinuteSlot {
    fn as_ref(&self) -> &str {
        match self {
            Self::TwelveAm => "12:00 AM",
            Self::TwelveThirtyAm => "12:30 AM",
            Self::OneAm => "1:00 AM",
            Self::OneThirtyAm => "1:30 AM",
            Self::TwoAm => "2:00 AM",
            Self::TwoThirtyAm => "2:30 AM",
            Self::ThreeAm => "3:00 AM",
            Self::ThreeThirtyAm => "3:30 AM",
            Self::FourAm => "4:00 AM",
            Self::FourThirtyAm => "4:30 AM",
            Self::FiveAm => "5:00 AM",
            Self::FiveThirtyAm => "5:30 AM",
            Self::SixAm => "6:00 AM",
            Self::SixThirtyAm => "6:30 AM",
            Self::SevenAm => "7:00 AM",
            Self::SevenThirtyAm => "7:30 AM",
            Self::EightAm => "8:00 AM",
            Self::EightThirtyAm => "8:30 AM",
            Self::NineAm => "9:00 AM",
            Self::NineThirtyAm => "9:30 AM",
            Self::TenAm => "10:00 AM",
            Self::TenThirtyAm => "10:30 AM",
            Self::ElevenAm => "11:00 AM",
            Self::ElevenThirtyAm => "11:30 AM",
            Self::TwelvePm => "12:00 PM",
            Self::TwelveThirtyPm => "12:30 PM",
            Self::OnePm => "1:00 PM",
            Self::OneThirtyPm => "1:30 PM",
            Self::TwoPm => "2:00 PM",
            Self::TwoThirtyPm => "2:30 PM",
            Self::ThreePm => "3:00 PM",
            Self::ThreeThirtyPm => "3:30 PM",
            Self::FourPm => "4:00 PM",
            Self::FourThirtyPm => "4:30 PM",
            Self::FivePm => "5:00 PM",
            Self::FiveThirtyPm => "5:30 PM",
            Self::SixPm => "6:00 PM",
            Self::SixThirtyPm => "6:30 PM",
            Self::SevenPm => "7:00 PM",
            Self::SevenThirtyPm => "7:30 PM",
            Self::EightPm => "8:00 PM",
            Self::EightThirtyPm => "8:30 PM",
            Self::NinePm => "9:00 PM",
            Self::NineThirtyPm => "9:30 PM",
            Self::TenPm => "10:00 PM",
            Self::TenThirtyPm => "10:30 PM",
            Self::ElevenPm => "11:00 PM",
            Self::ElevenThirtyPm => "11:30 PM",
        }
    }
}

impl ThirtyMinuteSlot {
    pub fn from_local_time(local_time: chrono::DateTime<chrono::Local>) -> Self {
        use chrono::Timelike;
        let hour = local_time.hour();
        let minute = local_time.minute() < 30;
        match (hour, minute) {
            (0, true) => Self::TwelveAm,
            (0, false) => Self::TwelveThirtyAm,
            (1, true) => Self::OneAm,
            (1, false) => Self::OneThirtyAm,
            (2, true) => Self::TwoAm,
            (2, false) => Self::TwoThirtyAm,
            (3, true) => Self::ThreeAm,
            (3, false) => Self::ThreeThirtyAm,
            (4, true) => Self::FourAm,
            (4, false) => Self::FourThirtyAm,
            (5, true) => Self::FiveAm,
            (5, false) => Self::FiveThirtyAm,
            (6, true) => Self::SixAm,
            (6, false) => Self::SixThirtyAm,
            (7, true) => Self::SevenAm,
            (7, false) => Self::SevenThirtyAm,
            (8, true) => Self::EightAm,
            (8, false) => Self::EightThirtyAm,
            (9, true) => Self::NineAm,
            (9, false) => Self::NineThirtyAm,
            (10, true) => Self::TenAm,
            (10, false) => Self::TenThirtyAm,
            (11, true) => Self::ElevenAm,
            (11, false) => Self::ElevenThirtyAm,
            (12, true) => Self::TwelvePm,
            (12, false) => Self::TwelveThirtyPm,
            (13, true) => Self::OnePm,
            (13, false) => Self::OneThirtyPm,
            (14, true) => Self::TwoPm,
            (14, false) => Self::TwoThirtyPm,
            (15, true) => Self::ThreePm,
            (15, false) => Self::ThreeThirtyPm,
            (16, true) => Self::FourPm,
            (16, false) => Self::FourThirtyPm,
            (17, true) => Self::FivePm,
            (17, false) => Self::FiveThirtyPm,
            (18, true) => Self::SixPm,
            (18, false) => Self::SixThirtyPm,
            (19, true) => Self::SevenPm,
            (19, false) => Self::SevenThirtyPm,
            (20, true) => Self::EightPm,
            (20, false) => Self::EightThirtyPm,
            (21, true) => Self::NinePm,
            (21, false) => Self::NineThirtyPm,
            (22, true) => Self::TenPm,
            (22, false) => Self::TenThirtyPm,
            (23, true) => Self::ElevenPm,
            (23, false) => Self::ElevenThirtyPm,
            _ => panic!("Invalid time"),
        }
    }

    pub const fn hours_minutes(&self) -> (u32, u32) {
        match self {
            Self::TwelveAm => (0, 0),
            Self::TwelveThirtyAm => (0, 30),
            Self::OneAm => (1, 0),
            Self::OneThirtyAm => (1, 30),
            Self::TwoAm => (2, 0),
            Self::TwoThirtyAm => (2, 30),
            Self::ThreeAm => (3, 0),
            Self::ThreeThirtyAm => (3, 30),
            Self::FourAm => (4, 0),
            Self::FourThirtyAm => (4, 30),
            Self::FiveAm => (5, 0),
            Self::FiveThirtyAm => (5, 30),
            Self::SixAm => (6, 0),
            Self::SixThirtyAm => (6, 30),
            Self::SevenAm => (7, 0),
            Self::SevenThirtyAm => (7, 30),
            Self::EightAm => (8, 0),
            Self::EightThirtyAm => (8, 30),
            Self::NineAm => (9, 0),
            Self::NineThirtyAm => (9, 30),
            Self::TenAm => (10, 0),
            Self::TenThirtyAm => (10, 30),
            Self::ElevenAm => (11, 0),
            Self::ElevenThirtyAm => (11, 30),
            Self::TwelvePm => (12, 0),
            Self::TwelveThirtyPm => (12, 30),
            Self::OnePm => (13, 0),
            Self::OneThirtyPm => (13, 30),
            Self::TwoPm => (14, 0),
            Self::TwoThirtyPm => (14, 30),
            Self::ThreePm => (15, 0),
            Self::ThreeThirtyPm => (15, 30),
            Self::FourPm => (16, 0),
            Self::FourThirtyPm => (16, 30),
            Self::FivePm => (17, 0),
            Self::FiveThirtyPm => (17, 30),
            Self::SixPm => (18, 0),
            Self::SixThirtyPm => (18, 30),
            Self::SevenPm => (19, 0),
            Self::SevenThirtyPm => (19, 30),
            Self::EightPm => (20, 0),
            Self::EightThirtyPm => (20, 30),
            Self::NinePm => (21, 0),
            Self::NineThirtyPm => (21, 30),
            Self::TenPm => (22, 0),
            Self::TenThirtyPm => (22, 30),
            Self::ElevenPm => (23, 0),
            Self::ElevenThirtyPm => (23, 30),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::TwelveAm,
            Self::TwelveThirtyAm,
            Self::OneAm,
            Self::OneThirtyAm,
            Self::TwoAm,
            Self::TwoThirtyAm,
            Self::ThreeAm,
            Self::ThreeThirtyAm,
            Self::FourAm,
            Self::FourThirtyAm,
            Self::FiveAm,
            Self::FiveThirtyAm,
            Self::SixAm,
            Self::SixThirtyAm,
            Self::SevenAm,
            Self::SevenThirtyAm,
            Self::EightAm,
            Self::EightThirtyAm,
            Self::NineAm,
            Self::NineThirtyAm,
            Self::TenAm,
            Self::TenThirtyAm,
            Self::ElevenAm,
            Self::ElevenThirtyAm,
            Self::TwelvePm,
            Self::TwelveThirtyPm,
            Self::OnePm,
            Self::OneThirtyPm,
            Self::TwoPm,
            Self::TwoThirtyPm,
            Self::ThreePm,
            Self::ThreeThirtyPm,
            Self::FourPm,
            Self::FourThirtyPm,
            Self::FivePm,
            Self::FiveThirtyPm,
            Self::SixPm,
            Self::SixThirtyPm,
            Self::SevenPm,
            Self::SevenThirtyPm,
            Self::EightPm,
            Self::EightThirtyPm,
            Self::NinePm,
            Self::NineThirtyPm,
            Self::TenPm,
            Self::TenThirtyPm,
            Self::ElevenPm,
            Self::ElevenThirtyPm,
        ]
    }

    pub fn naive_time(&self) -> chrono::NaiveTime {
        match self {
            Self::TwelveAm => chrono::NaiveTime::from_hms_opt(0, 0, 0),
            Self::TwelveThirtyAm => chrono::NaiveTime::from_hms_opt(0, 30, 0),
            Self::OneAm => chrono::NaiveTime::from_hms_opt(1, 0, 0),
            Self::OneThirtyAm => chrono::NaiveTime::from_hms_opt(1, 30, 0),
            Self::TwoAm => chrono::NaiveTime::from_hms_opt(2, 0, 0),
            Self::TwoThirtyAm => chrono::NaiveTime::from_hms_opt(2, 30, 0),
            Self::ThreeAm => chrono::NaiveTime::from_hms_opt(3, 0, 0),
            Self::ThreeThirtyAm => chrono::NaiveTime::from_hms_opt(3, 30, 0),
            Self::FourAm => chrono::NaiveTime::from_hms_opt(4, 0, 0),
            Self::FourThirtyAm => chrono::NaiveTime::from_hms_opt(4, 30, 0),
            Self::FiveAm => chrono::NaiveTime::from_hms_opt(5, 0, 0),
            Self::FiveThirtyAm => chrono::NaiveTime::from_hms_opt(5, 30, 0),
            Self::SixAm => chrono::NaiveTime::from_hms_opt(6, 0, 0),
            Self::SixThirtyAm => chrono::NaiveTime::from_hms_opt(6, 30, 0),
            Self::SevenAm => chrono::NaiveTime::from_hms_opt(7, 0, 0),
            Self::SevenThirtyAm => chrono::NaiveTime::from_hms_opt(7, 30, 0),
            Self::EightAm => chrono::NaiveTime::from_hms_opt(8, 0, 0),
            Self::EightThirtyAm => chrono::NaiveTime::from_hms_opt(8, 30, 0),
            Self::NineAm => chrono::NaiveTime::from_hms_opt(9, 0, 0),
            Self::NineThirtyAm => chrono::NaiveTime::from_hms_opt(9, 30, 0),
            Self::TenAm => chrono::NaiveTime::from_hms_opt(10, 0, 0),
            Self::TenThirtyAm => chrono::NaiveTime::from_hms_opt(10, 30, 0),
            Self::ElevenAm => chrono::NaiveTime::from_hms_opt(11, 0, 0),
            Self::ElevenThirtyAm => chrono::NaiveTime::from_hms_opt(11, 30, 0),
            Self::TwelvePm => chrono::NaiveTime::from_hms_opt(12, 0, 0),
            Self::TwelveThirtyPm => chrono::NaiveTime::from_hms_opt(12, 30, 0),
            Self::OnePm => chrono::NaiveTime::from_hms_opt(13, 0, 0),
            Self::OneThirtyPm => chrono::NaiveTime::from_hms_opt(13, 30, 0),
            Self::TwoPm => chrono::NaiveTime::from_hms_opt(14, 0, 0),
            Self::TwoThirtyPm => chrono::NaiveTime::from_hms_opt(14, 30, 0),
            Self::ThreePm => chrono::NaiveTime::from_hms_opt(15, 0, 0),
            Self::ThreeThirtyPm => chrono::NaiveTime::from_hms_opt(15, 30, 0),
            Self::FourPm => chrono::NaiveTime::from_hms_opt(16, 0, 0),
            Self::FourThirtyPm => chrono::NaiveTime::from_hms_opt(16, 30, 0),
            Self::FivePm => chrono::NaiveTime::from_hms_opt(17, 0, 0),
            Self::FiveThirtyPm => chrono::NaiveTime::from_hms_opt(17, 30, 0),
            Self::SixPm => chrono::NaiveTime::from_hms_opt(18, 0, 0),
            Self::SixThirtyPm => chrono::NaiveTime::from_hms_opt(18, 30, 0),
            Self::SevenPm => chrono::NaiveTime::from_hms_opt(19, 0, 0),
            Self::SevenThirtyPm => chrono::NaiveTime::from_hms_opt(19, 30, 0),
            Self::EightPm => chrono::NaiveTime::from_hms_opt(20, 0, 0),
            Self::EightThirtyPm => chrono::NaiveTime::from_hms_opt(20, 30, 0),
            Self::NinePm => chrono::NaiveTime::from_hms_opt(21, 0, 0),
            Self::NineThirtyPm => chrono::NaiveTime::from_hms_opt(21, 30, 0),
            Self::TenPm => chrono::NaiveTime::from_hms_opt(22, 0, 0),
            Self::TenThirtyPm => chrono::NaiveTime::from_hms_opt(22, 30, 0),
            Self::ElevenPm => chrono::NaiveTime::from_hms_opt(23, 0, 0),
            Self::ElevenThirtyPm => chrono::NaiveTime::from_hms_opt(23, 30, 0),
        }
        .unwrap_or_default()
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SlotPickerProps {
    pub slot: UseStateHandle<ThirtyMinuteSlot>,
}

#[function_component(ThirtyMinuteSlotPicker)]
fn slot_picker(props: &SlotPickerProps) -> Html {
    let onchange = {
        let slot = props.slot.setter();
        Callback::from(move |new_slot: Option<ThirtyMinuteSlot>| {
            if let Some(new_slot) = new_slot {
                slot.set(new_slot);
            }
        })
    };

    html! {
        <shady_minions::ui::Select<ThirtyMinuteSlot> class="w-full" {onchange} value={Some((*props.slot, props.slot.as_ref().to_string()))}>
            <shady_minions::ui::SelectTrigger<ThirtyMinuteSlot> label="Escoger hora" />
            <shady_minions::ui::SelectContent<ThirtyMinuteSlot>>
                { for ThirtyMinuteSlot::all().into_iter().map(|slot| {
                    html! {
                        <shady_minions::ui::SelectItem<ThirtyMinuteSlot> value={slot} label={slot.as_ref().to_string()} />
                    }
                })}
            </shady_minions::ui::SelectContent<ThirtyMinuteSlot>>
        </shady_minions::ui::Select<ThirtyMinuteSlot>>
    }
}

#[function_component(ConfirmingAppointment)]
fn confirming_appointment(props: &FormStepProps) -> Html {
    let navigator = yew_router::hooks::use_navigator();
    let confirmed_appointment_id = use_state(|| None::<String>);
    let history_ctx = crate::features::nostr_notes::use_history_data();

    // Extract slot and doctor from form state - AFTER all hooks
    let (slot_json, doctor_pubkey) = match &*props.form_step {
        AdminFormState::Confirming {
            doctor_pubkey,
            slot_json,
        } => (Some(slot_json.clone()), Some(doctor_pubkey.clone())),
        _ => (None, None),
    };

    let expected_slot: Option<paravida_models::ParavidaScheduleSlot> = slot_json
        .clone()
        .and_then(|json| serde_json::from_str(&json).ok());

    // Listen for matching appointment from HistoryStore context
    use_effect_with(
        (
            history_ctx.clone(),
            expected_slot.clone(),
            confirmed_appointment_id.clone(),
        ),
        move |(history_ctx, expected_slot, confirmed_id)| {
            // Don't re-run if we already found the appointment
            if confirmed_id.is_some() {
                return;
            }

            let Some(history_ctx) = history_ctx.as_ref() else {
                return;
            };
            let Some(expected_slot) = expected_slot.as_ref() else {
                return;
            };

            // Look for matching appointment in the store
            for item in &history_ctx.appointments {
                if item.appointment.start == expected_slot.start()
                    && item.appointment.end == expected_slot.end()
                    && item.appointment.room == expected_slot.room
                    && item.appointment.specialty == expected_slot.specialty
                    && item.appointment.status == paravida_models::ParavidaAppointmentStatus::Booked
                {
                    confirmed_id.setter().set(Some(item.id.clone()));
                    return;
                }
            }
        },
    );

    // Early return AFTER hooks if not in correct state
    if slot_json.is_none() || doctor_pubkey.is_none() {
        return html! {};
    }

    // Navigate to appointment detail when confirmed
    if let Some(appointment_id) = (*confirmed_appointment_id).clone() {
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
                    {"Cita Confirmada"}
                </paravida_components::typography::Highlight>
                <paravida_components::typography::P class="text-center text-muted">
                    {"La cita ha sido procesada exitosamente por el servidor."}
                </paravida_components::typography::P>
                <paravida_components::buttons::NormalButton
                    onclick={
                        let navigator = navigator.clone();
                        let appointment_id = appointment_id.clone();
                        Callback::from(move |_| {
                            if let Some(navigator) = navigator.as_ref() {
                                navigator.push(&crate::router::AppRoute::AppointmentDetail {
                                    id: appointment_id.clone()
                                });
                            }
                        })
                    }
                >
                    {"Ver Detalles de la Cita"}
                </paravida_components::buttons::NormalButton>
            </div>
        }
    } else {
        html! {
            <div class="flex flex-col gap-6 items-center justify-center py-8">
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
                    {"Procesando la cita..."}
                </paravida_components::typography::Highlight>
                <paravida_components::typography::P class="text-center text-muted">
                    {"El servidor está procesando la solicitud. Esto puede tomar unos momentos."}
                </paravida_components::typography::P>
            </div>
        }
    }
}

/// Hook to load doctors paginated for the form
/// Loads doctors in batches to avoid performance issues
#[hook]
fn use_load_all_doctors_for_form() {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let doctors_store = crate::features::nostr_notes::use_doctors();

    use_effect_with(
        (local_db.clone(), nostr_key.clone(), doctors_store.clone()),
        {
            move |(local_db, nostr_key, doctors_store)| {
                let Some(local_db) = local_db.as_ref() else {
                    return;
                };
                let Some(nostr_key) = nostr_key.as_ref() else {
                    return;
                };
                let Some(doctors_store) = doctors_store else {
                    return;
                };

                let local_db = local_db.clone();
                let nostr_key = nostr_key.clone();
                let doctors_store = doctors_store.clone();

                yew::platform::spawn_local(async move {
                    // Load first 50 doctors paginated - enough for most cases
                    match local_db
                        .get_doctors_paginated_filtered(None, 50, None, &nostr_key)
                        .await
                    {
                        Ok((doctors_notes, _, _)) => {
                            let doctors: Vec<crate::features::nostr_notes::DoctorItem> =
                                doctors_notes
                                    .into_iter()
                                    .filter_map(|(pubkey, note)| {
                                        let mutual_note =
                                            mutual_consent_notes::MutualConsentNote(note);
                                        let shared_document =
                                            mutual_note.view_shared_document(&nostr_key).ok()?;
                                        let practitioner =
                                            paravida_models::ParavidaPractitioner::from_salud_note(
                                                &shared_document,
                                            )
                                            .ok()?;
                                        Some(crate::features::nostr_notes::DoctorItem {
                                            pubkey,
                                            practitioner,
                                        })
                                    })
                                    .collect();

                            doctors_store.dispatch(
                                crate::features::nostr_notes::DoctorsAction::SetDoctors(doctors),
                            );
                        }
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Error loading doctors for form: {e:#?}").into(),
                            );
                        }
                    }
                });
            }
        },
    );
}
