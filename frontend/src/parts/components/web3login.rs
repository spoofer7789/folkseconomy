use yew::prelude::*;
use gloo_net::http::*;
use yew_ethereum_provider::{
    chain, AccountLabel, EthereumContextProvider, SwitchNetworkButton, ConnectButton, Ethereum,
};
use yew::hooks::{use_context, use_effect_with_deps};

#[function_component]
pub fn Web3Login() -> Html {
    let ethereum = use_context::<Ethereum>().expect("No Ethereum context found");

    use_effect_with_deps(
        |ethereum| {
            let handle = ethereum.subscribe_account_changed(Callback::from(move |accounts| {
                if let Some(account) = accounts.get(0) {
                    log::info!("Connected with public key: {:?}", account);
                } else {
                    log::warn!("No account found");
                }
            }));

            move || {
                ethereum.unsubscribe_account_changed(handle);
            }
        },
        ethereum.clone(),
    );

    html! {
        <div>
            <EthereumContextProvider>
                <ConnectButton>
                    <button>{"Connect"}</button>
                </ConnectButton>
                <SwitchNetworkButton chain={chain::ethereum()}/>
                <SwitchNetworkButton chain={chain::avalanche_testnet()}/>
                <AccountLabel />
            </EthereumContextProvider>
        </div>
    }
}
