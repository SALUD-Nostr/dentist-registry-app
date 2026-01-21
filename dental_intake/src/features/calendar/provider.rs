use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarData {
    pub appointments: Vec<paravida_models::ParavidaAppointment>,
    pub updated: u32,
}

#[allow(dead_code)]
pub enum CalendarAction {
    Updated,
}

impl Reducible for CalendarData {
    type Action = CalendarAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CalendarAction::Updated => std::rc::Rc::new(Self {
                appointments: self.appointments.clone(),
                updated: self.updated + 1,
            }),
        }
    }
}

pub type CalendarStore = UseReducerHandle<CalendarData>;

#[function_component(CalendarDataProvider)]
pub fn calendar_data_provider(props: &yew::html::ChildrenProps) -> Html {
    let ctx = use_reducer_eq(|| CalendarData {
        appointments: Vec::new(),
        updated: 0,
    });

    html! {
        <ContextProvider<CalendarStore> context={ctx}>
            { props.children.clone() }
        </ContextProvider<CalendarStore>>
    }
}

#[hook]
pub fn use_calendar_data() -> Option<CalendarStore> {
    use_context::<CalendarStore>()
}
