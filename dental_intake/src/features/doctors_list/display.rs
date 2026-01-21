use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct DoctorListItemProps {
    pub doctor: super::provider::DoctorListItem,
}

#[function_component(DoctorListItem)]
pub fn doctor_list_item(props: &DoctorListItemProps) -> Html {
    let specialty = props.doctor.practitioner.specialty.first().map_or_else(
        || "Especialidad".to_string(),
        paravida_models::ParavidaSpecialty::display_text,
    );

    let name = if props.doctor.practitioner.name.text().trim().is_empty() {
        "Nombre No Definido".to_string()
    } else {
        props.doctor.practitioner.name.text()
    };

    html! {
        <yew_router::components::Link<crate::router::Route>
            to={crate::router::Route::DoctorDetail { pubkey: props.doctor.pubkey.clone() }}
            classes="block w-full"
        >
            <paravida_components::Card class="!p-0 hover:shadow-lg transition-shadow cursor-pointer">
                <div class="flex flex-row size-full ">
                    <div class="self-stretch bg-primary p-6 rounded-l-xl flex justify-center items-center">
                        <paravida_components::icons::ParavidaLogoBlanco class="size-6 sm:size-9 md:size-12 mx-auto shrink-0" />
                    </div>
                    <div class="flex flex-row flex-1 justify-between p-6 sm:p-8 bg-background overflow-clip rounded-r-xl">
                        <div class="flex flex-col flex-1 text-start">
                            <paravida_components::typography::SubTitle>
                                { name }
                            </paravida_components::typography::SubTitle>
                            <paravida_components::typography::Highlight>
                                { specialty }
                            </paravida_components::typography::Highlight>
                        </div>
                        <div class="flex items-center">
                            <paravida_components::icons::ChevronLeft class="size-6 rotate-180 text-primary" />
                        </div>
                    </div>
                </div>
            </paravida_components::Card>
        </yew_router::components::Link<crate::router::Route>>
    }
}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct DoctorCardProps {
    pub doctor: super::provider::DoctorListItem,
}

#[function_component(DoctorCard)]
pub fn doctor_card(props: &DoctorCardProps) -> Html {
    let profile = &props.doctor.practitioner;

    let name = if profile.name.text().trim().is_empty() {
        "Nombre No Definido".to_string()
    } else {
        profile.name.text()
    };

    let specialty = profile.specialty.first().map_or_else(
        || "Especialidad".to_string(),
        paravida_models::ParavidaSpecialty::display_text,
    );

    let email = profile
        .contacts
        .iter()
        .find_map(|contact| {
            contact
                .system
                .eq(&paravida_models::PractitionerContactSystem::Email)
                .then_some(contact.value.clone())
        })
        .unwrap_or_else(|| "No especificado".to_string());

    let phone = profile
        .contacts
        .iter()
        .find_map(|contact| {
            contact
                .system
                .eq(&paravida_models::PractitionerContactSystem::Phone)
                .then_some(contact.value.clone())
        })
        .unwrap_or_else(|| "No especificado".to_string());

    let dui = profile
        .identification
        .iter()
        .find_map(|identification| {
            identification
                .system
                .eq(&paravida_models::IdentifierType::Dui)
                .then_some(identification.value.clone())
        })
        .unwrap_or_else(|| "No especificado".to_string());

    let jvpm = profile
        .identification
        .iter()
        .find_map(|identification| {
            identification
                .system
                .eq(&paravida_models::IdentifierType::Jvpm)
                .then_some(identification.value.clone())
        })
        .unwrap_or_else(|| "No especificado".to_string());

    html! {
        <paravida_components::Card class="!p-0 !max-w-3xl">
            <div class="flex flex-col sm:flex-row min-h-64 flex-1 ">
                <div class="flex-1 bg-primary flex justify-center items-center p-4 w-full sm:w-fit rounded-t-xl sm:rounded-l-xl sm:rounded-tr-none">
                    <paravida_components::icons::ParavidaLogoBlanco class="size-6 sm:size-9 md:size-12 mx-auto shrink-0" />
                </div>
                <div class="flex-5 p-6 gap-3 flex flex-col text-start items-start w-full">
                    <paravida_components::typography::SubTitle class="text-nowrap">
                        { name }
                    </paravida_components::typography::SubTitle>
                    <paravida_components::typography::Highlight class="text-start">
                        { specialty }
                    </paravida_components::typography::Highlight>
                    <div class="grid grid-cols-2 gap-6 mt-3 shrink-0 size-fit w-full">
                        <div class="flex flex-row gap-3 items-center col-span-2">
                            <paravida_components::icons::Email class="size-6 text-secondary shrink-0" />
                            <paravida_components::typography::P class="text-sm text-nowrap">
                                { email }
                            </paravida_components::typography::P>
                        </div>
                        <div class="flex flex-row gap-3 items-center col-span-2">
                            <paravida_components::icons::Phone class="size-6 text-secondary shrink-0" />
                            <paravida_components::typography::P class="text-sm text-nowrap">
                                { phone }
                            </paravida_components::typography::P>
                        </div>
                        <div class="flex flex-row gap-3 text-start items-center">
                            <paravida_components::icons::IdCard class="size-6 text-secondary shrink-0" />
                            <paravida_components::typography::P class="text-sm text-nowrap">
                                { dui }
                            </paravida_components::typography::P>
                        </div>
                        <div class="flex flex-row gap-3 text-start items-center">
                            <paravida_components::icons::Lanyard class="size-6 text-secondary shrink-0" />
                            <paravida_components::typography::P class="text-sm text-nowrap">
                                { jvpm }
                            </paravida_components::typography::P>
                        </div>
                    </div>
                </div>
            </div>
        </paravida_components::Card>
    }
}
