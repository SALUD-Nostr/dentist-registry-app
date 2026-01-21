use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoctorItem {
    pub pubkey: String,
    pub practitioner: paravida_models::ParavidaPractitioner,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoctorsState {
    pub doctors: Vec<DoctorItem>,
    pub updated: u32,
}

#[allow(dead_code)]
pub enum DoctorsAction {
    AddDoctor(DoctorItem),
    SetDoctors(Vec<DoctorItem>),
    Updated,
}

impl Reducible for DoctorsState {
    type Action = DoctorsAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut state = (*self).clone();
        match action {
            DoctorsAction::AddDoctor(doctor) => {
                state.doctors.retain(|d| d.pubkey != doctor.pubkey);
                state.doctors.push(doctor);
                state.updated += 1;
            }
            DoctorsAction::SetDoctors(doctors) => {
                state.doctors = doctors;
                state.updated += 1;
            }
            DoctorsAction::Updated => {
                state.updated += 1;
            }
        }
        std::rc::Rc::new(state)
    }
}

pub type DoctorsStore = UseReducerHandle<DoctorsState>;

#[function_component(DoctorsProvider)]
pub fn doctors_provider(props: &yew::html::ChildrenProps) -> HtmlResult {
    let state = use_reducer_eq(|| DoctorsState {
        doctors: Vec::new(),
        updated: 0,
    });

    Ok(html! {
        <ContextProvider<DoctorsStore> context={state}>
            {props.children.clone()}
        </ContextProvider<DoctorsStore>>
    })
}

#[hook]
pub fn use_doctors() -> Option<DoctorsStore> {
    use_context::<DoctorsStore>()
}

#[hook]
pub fn use_doctors_count() -> Option<usize> {
    let ctx = use_doctors();
    *use_memo(ctx, |ctx| ctx.as_ref().map(|ctx| ctx.doctors.len()))
}

#[hook]
pub fn use_all_doctors() -> Vec<DoctorItem> {
    let ctx = use_doctors();
    (*use_memo(ctx, |ctx| ctx.as_ref().map(|ctx| ctx.doctors.clone())))
        .clone()
        .unwrap_or_default()
}
