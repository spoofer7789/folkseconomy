use yew::prelude::*;

use yew_ethereum_provider::{
    chain, AccountLabel, EthereumContextProvider, SwitchNetworkButton,ConnectButton
};

#[function_component]
pub fn Web3Login() -> Html {
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