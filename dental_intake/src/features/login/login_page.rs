use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use nostr_minions::NostrSigner;

use crate::components::{Button, ButtonVariant, ButtonSize};
use crate::components::typography::{Subtitle, MutedText, Label, NormalText};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let nsec = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
    let show_generate_confirm = use_state(|| false);
    let password_input_ref = use_node_ref();
    let create_key = nostr_minions::use_create_local_key();

    let on_nsec_input = {
        let nsec = nsec.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            nsec.set(input.value());
        })
    };

    let on_submit = {
        let nsec = nsec.clone();
        let error = error.clone();
        let create_key = create_key.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(None);

            let nsec_value = (*nsec).clone();

            // Validate and create Nostr keypair
            match nsec_value.parse::<nostr_minions::nostro2_signer::keypair::NostrKeypair>() {
                Ok(mut keypair) => {
                    keypair.set_extractable(true);
                    create_key.emit(keypair);
                }
                Err(e) => {
                    error.set(Some(format!("Error al iniciar sesión: {e:#?}")));
                }
            }
        })
    };

    let on_generate_click = {
        let show_generate_confirm = show_generate_confirm.clone();
        Callback::from(move |_: MouseEvent| {
            show_generate_confirm.set(true);
        })
    };

    let on_confirm_generate = {
        let show_generate_confirm = show_generate_confirm.clone();
        let password_input_ref = password_input_ref.clone();
        let create_key = create_key.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Generate new keypair (extractable = true)
            let keypair = nostr_minions::nostro2_signer::keypair::NostrKeypair::generate(true);

            // Extract nsec for display/storage
            match keypair.nsec() {
                Ok(nsec_string) => {
                    // Set the password input value to trigger browser save
                    if let Some(input) = password_input_ref.cast::<HtmlInputElement>() {
                        input.set_value(&nsec_string);
                    }

                    // Use spawn_local to ensure password prompt happens before redirect
                    let create_key_clone = create_key.clone();
                    let show_generate_confirm_clone = show_generate_confirm.clone();
                    spawn_local(async move {
                        // Small delay to ensure form submission is processed by browser
                        gloo_timers::future::TimeoutFuture::new(200).await;

                        // Store the key (this will trigger redirect to app)
                        create_key_clone.emit(keypair);
                        show_generate_confirm_clone.set(false);
                    });
                }
                Err(e) => {
                    error.set(Some(format!("Error al exportar clave: {e:#?}")));
                    show_generate_confirm.set(false);
                }
            }
        })
    };

    let on_cancel_generate = {
        let show_generate_confirm = show_generate_confirm.clone();
        Callback::from(move |_: MouseEvent| {
            show_generate_confirm.set(false);
        })
    };

    html! {
        <div class="flex h-screen w-screen items-center justify-center bg-gradient-to-br from-primary/10 to-accent/10">
            <shady_minions::ui::Card class="w-full max-w-md p-8">
                <div class="flex flex-col items-center gap-6 mb-6">
                    <crate::components::Logo class="size-20 text-primary" />
                    <div class="text-center">
                        <Subtitle class="mb-2">
                            {"Portal Salud"}
                        </Subtitle>
                        <MutedText>
                            {"Ingresa tu clave privada (nsec) para acceder"}
                        </MutedText>
                    </div>
                </div>

                <form onsubmit={on_submit} class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <Label for_id="nsec" class="text-foreground">
                            {"Clave Privada (nsec)"}
                        </Label>
                        <input
                            id="nsec"
                            type="password"
                            placeholder="nsec1..."
                            value={(*nsec).clone()}
                            oninput={on_nsec_input}
                            class="px-3 py-2 border border-input rounded-md bg-background text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent"
                        />
                    </div>

                    if let Some(error_msg) = (*error).clone() {
                        <div class="p-3 rounded-md bg-destructive/10 border border-destructive/20 text-destructive text-sm">
                            { error_msg }
                        </div>
                    }

                    <Button
                        variant={ButtonVariant::Primary}
                        size={ButtonSize::Medium}
                        button_type="submit".to_string()
                        full_width={true}
                    >
                        {"Iniciar Sesión"}
                    </Button>
                </form>

                <div class="mt-6 pt-6 border-t border-border">
                    <Button
                        variant={ButtonVariant::Secondary}
                        size={ButtonSize::Medium}
                        onclick={Some(on_generate_click)}
                        full_width={true}
                    >
                        {"Generar Nueva Clave"}
                    </Button>
                </div>

                <div class="mt-4 text-center">
                    <MutedText size="text-xs text-muted">
                        {"Tu clave privada se almacena de forma segura en tu navegador"}
                    </MutedText>
                </div>
            </shady_minions::ui::Card>

            // Confirmation Modal
            if *show_generate_confirm {
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
                    <shady_minions::ui::Card class="w-full max-w-lg p-6 m-4">
                        <form onsubmit={on_confirm_generate}>
                            <div class="flex flex-col gap-4">
                                <div class="flex items-start gap-3">
                                    <div class="flex-shrink-0 mt-1">
                                        <svg class="size-6 text-amber-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                        </svg>
                                    </div>
                                    <div class="flex-1">
                                        <Subtitle class="mb-2 text-foreground">
                                            {"⚠️ Importante: Guarda tu Clave Privada"}
                                        </Subtitle>
                                        <div class="space-y-3 text-sm text-foreground">
                                            <NormalText>
                                                {"Estás a punto de generar una nueva clave privada. Esta clave es la ÚNICA forma de acceder a tu cuenta."}
                                            </NormalText>
                                            <div class="p-3 bg-amber-50 dark:bg-amber-950/20 border border-amber-200 dark:border-amber-900 rounded-md">
                                                <NormalText class="font-semibold text-amber-900 dark:text-amber-100 mb-2">
                                                    {"Debes guardar tu clave en uno de estos lugares seguros:"}
                                                </NormalText>
                                                <ul class="list-disc list-inside space-y-1 text-amber-800 dark:text-amber-200">
                                                    <li>{"Gestor de contraseñas del navegador (recomendado)"}</li>
                                                    <li>{"Gestor de contraseñas externo (1Password, Bitwarden, etc.)"}</li>
                                                    <li>{"Escrita físicamente en un lugar seguro"}</li>
                                                </ul>
                                            </div>
                                            <NormalText class="font-semibold text-destructive">
                                                {"Si pierdes tu clave privada, perderás el acceso a tu cuenta PERMANENTEMENTE."}
                                            </NormalText>
                                            <NormalText class="text-muted-foreground">
                                                {"Después de generar la clave, tu navegador te pedirá guardar la contraseña. Asegúrate de aceptar."}
                                            </NormalText>
                                        </div>
                                    </div>
                                </div>

                                // Hidden inputs to trigger browser password save
                                // These are invisible but trigger the browser's "Save Password?" prompt
                                <input
                                    type="text"
                                    name="username"
                                    value="salud-dental"
                                    autocomplete="username"
                                    class="sr-only"
                                    aria-hidden="true"
                                    tabindex="-1"
                                />
                                <input
                                    ref={password_input_ref.clone()}
                                    type="password"
                                    name="password"
                                    id="generated-key"
                                    autocomplete="new-password"
                                    class="sr-only"
                                    aria-hidden="true"
                                    tabindex="-1"
                                />

                                <div class="flex gap-3 mt-4">
                                    <Button
                                        variant={ButtonVariant::Outline}
                                        size={ButtonSize::Medium}
                                        onclick={Some(on_cancel_generate)}
                                        class="flex-1"
                                    >
                                        {"Cancelar"}
                                    </Button>
                                    <Button
                                        variant={ButtonVariant::Primary}
                                        size={ButtonSize::Medium}
                                        button_type="submit".to_string()
                                        class="flex-1"
                                    >
                                        {"Entiendo, Generar Clave"}
                                    </Button>
                                </div>
                            </div>
                        </form>
                    </shady_minions::ui::Card>
                </div>
            }
        </div>
    }
}
