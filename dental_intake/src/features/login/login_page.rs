use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let nsec = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
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

    html! {
        <div class="flex h-screen w-screen items-center justify-center bg-gradient-to-br from-primary/10 to-accent/10">
            <shady_minions::ui::Card class="w-full max-w-md p-8">
                <div class="flex flex-col items-center gap-6 mb-6">
                    <crate::components::Logo class="size-20 text-primary" />
                    <div class="text-center">
                        <h1 class="text-2xl font-bold text-foreground mb-2">
                            {"Salud Dental"}
                        </h1>
                        <p class="text-sm text-muted-foreground">
                            {"Ingresa tu clave privada (nsec) para acceder"}
                        </p>
                    </div>
                </div>

                <form onsubmit={on_submit} class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <label for="nsec" class="text-sm font-medium text-foreground">
                            {"Clave Privada (nsec)"}
                        </label>
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

                    <button
                        type="submit"
                        class="px-4 py-2 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 transition-colors"
                    >
                        {"Iniciar Sesión"}
                    </button>
                </form>

                <div class="mt-6 pt-6 border-t border-border text-center">
                    <p class="text-xs text-muted-foreground">
                        {"Tu clave privada se almacena de forma segura en tu navegador"}
                    </p>
                </div>
            </shady_minions::ui::Card>
        </div>
    }
}
