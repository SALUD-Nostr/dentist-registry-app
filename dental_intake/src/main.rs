#![warn(clippy::all, clippy::style, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::future_not_send,
    clippy::type_repetition_in_bounds
)]

use yew::prelude::*;

#[cfg(not(feature = "production"))]
pub const PARAVIDA_PUBKEY: &str =
    "a381a50f761c28e1dffba80639c64d86260f3520b2af3119a5dec3be9dd8e516";

#[cfg(feature = "production")]
pub const PARAVIDA_PUBKEY: &str =
    "9e3235a798c7decdaed2716eb883d4dde738c81b512babe7bc21c9d94b6417a1";

mod components;
mod constants;
mod error;
mod features;
mod router;
mod storage;
// pub(crate) mod shared; // TODO: Re-enable after Nostr integration restored

#[function_component(App)]
fn app() -> Html {
    html! {
        <nostr_minions::NostrIdProvider>
            <features::login::LoginWrapper>
                <yew_router::router::BrowserRouter>
                    <yew::suspense::Suspense fallback={html!{<Splash />}}>
                        <storage::StorageProvider>
                            <router::AppRouter />
                        </storage::StorageProvider>
                    </yew::suspense::Suspense>
                </yew_router::router::BrowserRouter>
            </features::login::LoginWrapper>
        </nostr_minions::NostrIdProvider>
    }
}

#[function_component(Splash)]
fn splash() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center size-screen bg-primary">
            <crate::components::Logo class="size-32 mx-auto text-white" />
        </div>
    }
}

fn main() {
    let Some(document) = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("app"))
    else {
        web_sys::console::error_1(&"App not found".into());
        return;
    };
    yew::Renderer::<App>::with_root(document).render();
}

// TODO: Re-enable Nostr functions after initial refactor
// #[hook]
// pub fn use_send_server_message() -> Callback<nostr_minions::nostro2::NostrNote> {
//     let relay_ctx = nostr_minions::use_nostr_relay_pool();
//     let keypair = nostr_minions::use_nostr_key();
//     Callback::from(move |server_message: nostr_minions::nostro2::NostrNote| {
//         let mut server_message = server_message;
//         let Some(keypair) = keypair.as_ref() else {
//             return;
//         };
//         if let Err(e) = keypair.sign_note(&mut server_message) {
//             web_sys::console::error_1(&format!("{e:#?}").into());
//             return;
//         }
//         match encrypt_server_message(&server_message, PARAVIDA_PUBKEY) {
//             Ok(encrypted_server_message) => {
//                 relay_ctx.send(encrypted_server_message);
//             }
//             Err(e) => {
//                 web_sys::console::error_1(&e);
//             }
//         }
//     })
// }

// fn encrypt_server_message(
//     nostr_note: &nostro2::NostrNote,
//     recipient: &str,
// ) -> Result<nostro2::NostrNote, web_sys::wasm_bindgen::JsValue> {
//     use nostro2::NostrSigner;
//     let mut server_message = nostro2::NostrNote {
//         content: nostr_note
//             .serialize()
//             .map_err(|err| web_sys::wasm_bindgen::JsValue::from_str(&format!("{err:#?}")))?,
//         kind: constants::magic_numbers::SERVER_MESSAGE_KIND,
//         ..Default::default()
//     };
//     server_message
//         .tags
//         .add_pubkey_tag(recipient, Some("wss://relay.illuminodes.com"));
//     let ephemeral_key = nostr_minions::nostro2_signer::keypair::NostrKeypair::generate(true);
//     ephemeral_key
//         .sign_encrypted_note(
//             &mut server_message,
//             recipient,
//             &nostr_minions::nostro2_signer::keypair::EncryptionScheme::Nip44,
//         )
//         .map_err(|err| web_sys::wasm_bindgen::JsValue::from_str(&format!("{err:#?}")))?;
//     Ok(server_message)
// }
