use yew::prelude::*;

const DOCTORS_PER_PAGE: usize = 10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoctorListItem {
    pub pubkey: String,
    pub practitioner: paravida_models::ParavidaPractitioner,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoctorsListState {
    pub current_page_doctors: Vec<DoctorListItem>, // Only current page
    pub current_page: usize,
    pub total_count: usize, // Total doctors in DB (or filtered count when searching)
    pub search_query: String, // User input for search
    pub active_search: String, // Currently applied search filter
    pub loading: bool,
}

pub enum DoctorsListAction {
    SetPageDoctors(Vec<DoctorListItem>, usize), // Replace current page doctors + total count
    SetPage(usize),
    SetSearchQuery(String), // Update search input
    ApplySearch(String),    // Apply search and reset to page 0
    SetLoading(bool),
    // SetTotalCount(usize),
}

impl Reducible for DoctorsListState {
    type Action = DoctorsListAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut state = (*self).clone();
        match action {
            DoctorsListAction::SetPageDoctors(doctors, total) => {
                state.current_page_doctors = doctors;
                state.total_count = total;
                state.loading = false;
            }
            DoctorsListAction::SetPage(page) => {
                state.current_page = page;
            }
            DoctorsListAction::SetSearchQuery(query) => {
                state.search_query = query;
            }
            DoctorsListAction::ApplySearch(query) => {
                state.active_search = query;
                state.current_page = 0;
            }
            DoctorsListAction::SetLoading(loading) => {
                state.loading = loading;
            } // DoctorsListAction::SetTotalCount(count) => {
              //     state.total_count = count;
              // }
        }
        std::rc::Rc::new(state)
    }
}

pub type DoctorsListStore = UseReducerHandle<DoctorsListState>;

#[function_component(DoctorsListProvider)]
pub fn doctors_list_provider(props: &yew::html::ChildrenProps) -> Html {
    let state = use_reducer(|| DoctorsListState {
        current_page_doctors: Vec::new(),
        current_page: 0,
        total_count: 0,
        search_query: String::new(),
        active_search: String::new(),
        loading: true,
    });

    html! {
        <ContextProvider<DoctorsListStore> context={state}>
            {props.children.clone()}
        </ContextProvider<DoctorsListStore>>
    }
}

#[hook]
pub fn use_doctors_list() -> DoctorsListStore {
    use_context::<DoctorsListStore>().expect("DoctorsListProvider not found")
}

/// Load doctors for current page with optional search filter
#[hook]
pub fn use_load_doctors() {
    let local_db = crate::local_db::use_local_idb_manager();
    let nostr_key = nostr_minions::use_nostr_key();
    let sync_status = crate::features::nostr_notes::use_sync_status();
    let doctors_list = use_doctors_list();

    // Load current page whenever page or active_search changes
    let current_page = doctors_list.current_page;
    let active_search = doctors_list.active_search.clone();

    use_effect_with(
        (
            local_db.clone(),
            nostr_key.clone(),
            sync_status.clone(),
            current_page,
            active_search,
        ),
        {
            let doctors_list = doctors_list.clone();
            move |(local_db, nostr_key, _sync_status, page, search)| {
                doctors_list.dispatch(DoctorsListAction::SetLoading(true));

                let Some(local_db) = local_db.as_ref() else {
                    return;
                };
                let Some(nostr_key) = nostr_key.as_ref() else {
                    return;
                };

                let local_db = local_db.clone();
                let nostr_key = nostr_key.clone();
                let doctors_list = doctors_list.clone();
                let page = *page;
                let search_query = if search.trim().is_empty() {
                    None
                } else {
                    Some(search.clone())
                };

                yew::platform::spawn_local(async move {
                    // Calculate how many doctors to skip based on page number
                    let skip = page * DOCTORS_PER_PAGE;

                    // Fetch doctors using filtered pagination
                    let (doctors_notes, _, total_matching) = match local_db
                        .get_doctors_paginated_filtered(
                            None,
                            skip + DOCTORS_PER_PAGE,
                            search_query,
                            &nostr_key,
                        )
                        .await
                    {
                        Ok(result) => result,
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Error loading doctors page: {e:#?}").into(),
                            );
                            doctors_list.dispatch(DoctorsListAction::SetPageDoctors(Vec::new(), 0));
                            return;
                        }
                    };

                    let doctors: Vec<DoctorListItem> = doctors_notes
                        .into_iter()
                        .skip(skip)
                        .take(DOCTORS_PER_PAGE)
                        .filter_map(|(pubkey, note)| {
                            let mutual_note = mutual_consent_notes::MutualConsentNote(note);
                            let shared_document =
                                mutual_note.view_shared_document(&nostr_key).ok()?;
                            let practitioner =
                                paravida_models::ParavidaPractitioner::from_salud_note(
                                    &shared_document,
                                )
                                .ok()?;
                            Some(DoctorListItem {
                                pubkey,
                                practitioner,
                            })
                        })
                        .collect();

                    doctors_list
                        .dispatch(DoctorsListAction::SetPageDoctors(doctors, total_matching));
                });
            }
        },
    );
}

pub const fn total_pages(total_count: usize) -> usize {
    total_count.div_ceil(DOCTORS_PER_PAGE)
}
