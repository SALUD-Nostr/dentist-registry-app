use yew::prelude::*;

#[function_component(SyncStatus)]
pub fn sync_status() -> Html {
    let sync_status = crate::features::nostr_notes::use_sync_status();

    let Some(sync_status) = sync_status else {
        return html! {};
    };

    match *sync_status {
        crate::features::nostr_notes::SyncStatus::Syncing => {
            html! {
                <div class="flex items-center gap-2 px-3 py-2 rounded-lg">
                    <svg
                        class="size-4 text-secondary animate-spin"
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
                    <span class="text-sm text-secondary font-medium">
                        {"Sincronizado"}
                    </span>
                </div>
            }
        }
        crate::features::nostr_notes::SyncStatus::Synced => {
            html! {
                <div class="flex items-center gap-2 px-3 py-2 rounded-lg">
                    <svg
                        class="size-4 text-primary"
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
                    <span class="text-sm text-primary font-medium">{"Sincronizado"}</span>
                </div>
            }
        }
        crate::features::nostr_notes::SyncStatus::Disconnected => {
            html! {
                <div class="flex items-center gap-2 px-3 py-2 bg-red-50 border border-red-200 rounded-lg">
                    <svg
                        class="size-4 text-red-600"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10"/>
                        <line x1="15" y1="9" x2="9" y2="15"/>
                        <line x1="9" y1="9" x2="15" y2="15"/>
                    </svg>
                    <span class="text-sm text-red-600 font-medium">{"Desconectado"}</span>
                </div>
            }
        }
    }
}
