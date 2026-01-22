//! Encounters schedule (calendar view)

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(EncountersSchedule)]
pub fn encounters_schedule() -> Html {
    let navigator = use_navigator().unwrap();

    let handle_new_encounter = {
        let navigator = navigator;
        Callback::from(move |_| {
            navigator.push(&crate::router::Route::EncounterNew);
        })
    };

    html! {
        <div class="flex flex-col size-full p-4 md:p-8">
            <div class="max-w-7xl mx-auto w-full h-full flex flex-col">
                <div class="flex justify-between items-center mb-6">
                    <h1 class="text-3xl font-bold">{"Calendario de Citas"}</h1>
                    <button
                        onclick={handle_new_encounter}
                        class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 flex items-center gap-2"
                    >
                        <crate::components::Plus class="size-5" />
                        {"Nueva Cita"}
                    </button>
                </div>

                <shady_minions::ui::Card class="flex-1 flex items-center justify-center">
                    <div class="text-center py-12">
                        <crate::components::Calendar class="size-20 mx-auto mb-4 opacity-50 text-muted" />
                        <p class="text-lg text-muted-foreground">{"Vista de calendario - Próximamente"}</p>
                        <p class="text-sm mt-2 text-muted">{"Esta pantalla mostrará un calendario con todas las citas programadas"}</p>
                    </div>
                </shady_minions::ui::Card>
            </div>
        </div>
    }
}
