use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let nsec = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
    let create_key = nostr_minions::use_create_local_key();

    let on_nsec_input = {
        let nsec = nsec.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
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
        <div class="flex h-screen w-screen items-center justify-center bg-muted/10">
            <paravida_components::Card class="max-w-md">
                <div class="flex flex-col items-center gap-4">
                    <paravida_components::icons::ParavidaLogo class="size-16" />
                    <paravida_components::typography::H2>{"Panel de Administración"}</paravida_components::typography::H2>
                    <paravida_components::typography::P class="text-center text-muted">
                        {"Ingresa tu clave privada (nsec) para acceder"}
                    </paravida_components::typography::P>
                </div>

                <form onsubmit={on_submit} class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <label for="nsec" class="text-sm font-medium">
                            {"Clave Privada (nsec)"}
                        </label>
                        <paravida_components::inputs::Input
                            id="nsec"
                            input_type="password"
                            placeholder="nsec1..."
                            value={(*nsec).clone()}
                            oninput={on_nsec_input}
                        />
                    </div>

                    if let Some(error_msg) = (*error).clone() {
                        <paravida_components::alerts::Alert variant="error">
                            { error_msg }
                        </paravida_components::alerts::Alert>
                    }

                    <paravida_components::buttons::Button
                        button_type="submit"
                        variant="primary"
                        class="w-full"
                    >
                        {"Iniciar Sesión"}
                    </paravida_components::buttons::Button>
                </form>
            </paravida_components::Card>
        </div>
    }
}
