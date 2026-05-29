use gloo_console::log;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

use crate::components::typography::{MutedText, Subtitle, Title};

/// Copy `text` to the system clipboard via the browser Clipboard API.
fn copy_to_clipboard(text: String) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let clipboard = window.navigator().clipboard();
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = JsFuture::from(clipboard.write_text(&text)).await {
            log!(format!("Error copying to clipboard: {:?}", e));
        }
    });
}

#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    let keypair = nostr_minions::use_nostr_key()
        .expect("SettingsPage is inside LoginWrapper, key is always Some");

    // Clone the keypair and mark as extractable for backup/recovery purposes
    let mut extractable_keypair = keypair.clone();
    extractable_keypair.set_extractable(true);

    let npub = extractable_keypair.npub().unwrap_or_else(|e| {
        log!(format!("Error getting npub: {:?}", e));
        "Error".to_string()
    });

    let nsec = extractable_keypair.nsec().unwrap_or_else(|e| {
        log!(format!("Error getting nsec: {:?}", e));
        "Error - no se pudo extraer la clave".to_string()
    });

    let mnemonic = extractable_keypair
        .mnemonic(nostr_minions::nostro2_signer::Language::Spanish)
        .unwrap_or_else(|e| {
            log!(format!("Error getting mnemonic: {:?}", e));
            "Error - no se pudo extraer la frase".to_string()
        });

    log!(format!("Keys extracted - nsec length: {}, mnemonic words: {}", nsec.len(), mnemonic.split_whitespace().count()));

    let show_nsec = use_state(|| false);
    let show_mnemonic = use_state(|| false);
    // Which key was most recently copied, for transient "Copiado" feedback.
    let copied = use_state(|| None::<&'static str>);

    let toggle_nsec = {
        let show_nsec = show_nsec.clone();
        Callback::from(move |_: MouseEvent| show_nsec.set(!*show_nsec))
    };
    let toggle_mnemonic = {
        let show_mnemonic = show_mnemonic.clone();
        Callback::from(move |_: MouseEvent| show_mnemonic.set(!*show_mnemonic))
    };

    // Build a copy handler for a given key, tagged with a label so the UI can
    // show which one was copied.
    let make_copy = {
        let copied = copied.clone();
        move |label: &'static str, value: String| {
            let copied = copied.clone();
            Callback::from(move |_: MouseEvent| {
                copy_to_clipboard(value.clone());
                copied.set(Some(label));
            })
        }
    };

    let mnemonic_words: Vec<&str> = mnemonic.split_whitespace().collect();

    html! {
        <div class="flex flex-col size-full p-4 gap-6">
            <div class="flex flex-col gap-2">
                <Title>
                    {"Ajustes"}
                </Title>
                <MutedText>
                    {"Administra tu identidad y claves de recuperacion"}
                </MutedText>
            </div>

            <div class="flex flex-col gap-4 max-w-3xl w-full">
                // Public Key
                <shady_minions::ui::Card class="!border-0 !shadow-none">
                    <div class="p-6">
                        <div class="flex items-center justify-between mb-2">
                            <Subtitle>{"Clave Publica"}</Subtitle>
                            <button
                                onclick={make_copy("npub", npub.clone())}
                                class="px-2 py-1 rounded-md text-sm hover:bg-muted/30 transition-colors cursor-pointer text-muted"
                            >
                                {if *copied == Some("npub") { "Copiado ✓" } else { "Copiar" }}
                            </button>
                        </div>
                        <MutedText class="mb-3">{"Tu identidad publica en la red Nostr."}</MutedText>
                        <code class="block w-full break-all rounded-lg bg-muted/30 p-3 text-sm font-mono text-foreground border border-muted">
                            {&npub}
                        </code>
                    </div>
                </shady_minions::ui::Card>

                // Private Key (nsec)
                <shady_minions::ui::Card class="!border-0 !shadow-none">
                    <div class="p-6">
                        <div class="flex items-center justify-between mb-2">
                            <Subtitle>{"Clave Privada"}</Subtitle>
                            <div class="flex items-center gap-1">
                                <button
                                    onclick={make_copy("nsec", nsec.clone())}
                                    class="px-2 py-1 rounded-md text-sm hover:bg-muted/30 transition-colors cursor-pointer text-muted"
                                >
                                    {if *copied == Some("nsec") { "Copiado ✓" } else { "Copiar" }}
                                </button>
                                <button
                                    onclick={toggle_nsec}
                                    class="p-1.5 rounded-md hover:bg-muted/30 transition-colors cursor-pointer"
                                    title={if *show_nsec { "Ocultar" } else { "Mostrar" }}
                                >
                                    if *show_nsec {
                                        <crate::components::EyeOff class="size-5 text-muted" />
                                    } else {
                                        <crate::components::Eye class="size-5 text-muted" />
                                    }
                                </button>
                            </div>
                        </div>
                        <MutedText class="mb-3">{"Tu clave secreta en formato nsec. No la compartas con nadie."}</MutedText>
                        <code class="block w-full break-all rounded-lg bg-muted/30 p-3 text-sm font-mono text-foreground border border-muted">
                            if *show_nsec {
                                {&nsec}
                            } else {
                                {"••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••"}
                            }
                        </code>
                    </div>
                </shady_minions::ui::Card>

                // Mnemonic (24 words)
                <shady_minions::ui::Card class="!border-0 !shadow-none">
                    <div class="p-6">
                        <div class="flex items-center justify-between mb-2">
                            <Subtitle>{"Frase de Recuperacion"}</Subtitle>
                            <div class="flex items-center gap-1">
                                <button
                                    onclick={make_copy("mnemonic", mnemonic.clone())}
                                    class="px-2 py-1 rounded-md text-sm hover:bg-muted/30 transition-colors cursor-pointer text-muted"
                                >
                                    {if *copied == Some("mnemonic") { "Copiado ✓" } else { "Copiar" }}
                                </button>
                                <button
                                    onclick={toggle_mnemonic}
                                    class="p-1.5 rounded-md hover:bg-muted/30 transition-colors cursor-pointer"
                                    title={if *show_mnemonic { "Ocultar" } else { "Mostrar" }}
                                >
                                    if *show_mnemonic {
                                        <crate::components::EyeOff class="size-5 text-muted" />
                                    } else {
                                        <crate::components::Eye class="size-5 text-muted" />
                                    }
                                </button>
                            </div>
                        </div>
                        <MutedText class="mb-3">{"24 palabras que te permiten recuperar tu cuenta. Guardalas en un lugar seguro."}</MutedText>
                        if *show_mnemonic {
                            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
                                { for mnemonic_words.iter().enumerate().map(|(i, word)| html! {
                                    <div class="rounded-lg bg-muted/30 px-3 py-2 text-sm font-mono text-foreground border border-muted">
                                        <span class="text-muted mr-1">{ format!("{}.", i + 1) }</span>
                                        { word }
                                    </div>
                                })}
                            </div>
                        } else {
                            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
                                { for (0..24).map(|_| html! {
                                    <div class="rounded-lg bg-muted/30 px-3 py-2 text-sm font-mono text-muted border border-muted">
                                        {"••••••"}
                                    </div>
                                })}
                            </div>
                        }
                    </div>
                </shady_minions::ui::Card>

                // Warning
                <div class="rounded-lg border border-amber-300 bg-amber-50 p-4">
                    <p class="text-sm font-semibold text-amber-800 mb-1">{"⚠️ Importante"}</p>
                    <MutedText class="text-amber-700">
                        {"Guarda tu clave privada y frase de recuperacion en un lugar seguro. Si pierdes acceso a tu cuenta, solo podras recuperarla con esta informacion."}
                    </MutedText>
                </div>
            </div>
        </div>
    }
}
