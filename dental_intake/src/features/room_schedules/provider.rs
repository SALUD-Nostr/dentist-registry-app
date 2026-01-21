use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RoomSchedule {
    pub room: paravida_models::ParavidaRoom,
    pub slots: Vec<paravida_models::ParavidaScheduleSlot>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct RoomSchedulesState {
    schedules: Vec<RoomSchedule>,
}

impl RoomSchedulesState {
    pub fn find_schedule(&self, room: paravida_models::ParavidaRoom) -> Option<&RoomSchedule> {
        self.schedules.iter().find(|schedule| schedule.room == room)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum RoomSchedulesAction {
    AddRoomSchedule(RoomSchedule),
}

impl Reducible for RoomSchedulesState {
    type Action = RoomSchedulesAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new_state = (*self).clone();
        match action {
            RoomSchedulesAction::AddRoomSchedule(schedule) => {
                new_state
                    .schedules
                    .retain(|schedules| schedules.room != schedule.room);
                new_state.schedules.push(schedule);
            }
        }
        std::rc::Rc::new(new_state)
    }
}

type RoomSchedules = UseReducerHandle<RoomSchedulesState>;

#[function_component(RoomSchedulesProvider)]
pub fn room_schedules_provider(props: &yew::html::ChildrenProps) -> Html {
    let nostr_relay = nostr_minions::use_nostr_relay_pool();

    let state = use_reducer(|| RoomSchedulesState {
        schedules: Vec::<RoomSchedule>::new(),
    });

    let nostr_relay_clone = nostr_relay.clone();
    use_effect_with((), move |()| {
        let filter = nostro2::NostrSubscription {
            kinds: vec![crate::constants::magic_numbers::SCHEDULE_NOTE_KIND].into(),
            authors: vec![crate::PARAVIDA_PUBKEY.to_string()].into(),
            ..Default::default()
        };
        let _ = nostr_relay_clone.send(filter);
    });

    let dispatch = state.dispatcher();
    use_effect_with(nostr_relay.last_note.clone(), move |last_note| {
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
        let Some(room_tag) = last_note.tags.find_tags("d").into_iter().next() else {
            return;
        };
        let Ok(slots) =
            serde_json::from_str::<Vec<paravida_models::ParavidaScheduleSlot>>(&last_note.content)
        else {
            return;
        };
        let Ok(room) = room_tag.as_str().parse() else {
            return;
        };
        let schedule = RoomSchedule { room, slots };
        dispatch.dispatch(RoomSchedulesAction::AddRoomSchedule(schedule));
    });

    html! {
        <ContextProvider<RoomSchedules> context={state}>
            {props.children.clone()}
        </ContextProvider<RoomSchedules>>
    }
}

#[hook]
pub fn use_room_schedule() -> UseReducerHandle<RoomSchedulesState> {
    use_context::<RoomSchedules>().expect("RoomSchedulesProvider not found")
}
