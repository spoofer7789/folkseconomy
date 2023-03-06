use std::fmt::format;
use crate::parts::components::login::LoginPage;
use crate::parts::components::createaccount::{CreateAccount};
use crate::parts::components::web3login::{Web3Login};
use crate::parts::pages::notfound::NotFound;
use yew::prelude::*;
use yew_router::{prelude::*, navigator};
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/web3_sign_in")]
    Web3Login,
    #[at("/createaccount")]
    CreateAccount,
    #[at("/search")]
    SearchRoot,
    #[at("/*search")]
    Search,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound
}
#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings")] //Create an Account for 
    GeneralSettings, 
    #[at("/settings/wallets")]
    Wallets,
    #[at("/settings/mycontracts")]
    MyContracts,
    #[at("/settings/privacy")]
    Privacy,
}
//User signs an item. Request to be anonymous or create username Users can buy and trade 
// user names
//User doesn't have to create an account although they can.
//UI does not require an account
//Making offers requires signing which builds a profile for you. If the user does this
//We will message them to create an Account

#[derive(Clone, Routable, PartialEq)]
enum SearchRoute {
    #[at("/search/*search")]
    Hashtag {search: String},
    #[at("/search/*profile")]
    Profile {profile: String},
    #[at("/search/*info")]
    Info {info: String},
    #[at("/search/trending")]
    Trending
}
//

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<div>  <h1> {"Folkseconomy"} </h1><Navbar/>
        </div>},
        MainRoute::Login => html! {<div><LoginPage/></div>},
        MainRoute::Web3Login => html! {<div><Web3Login/></div>},
        MainRoute::CreateAccount => html! {<div><Navbar/> <CreateAccount/></div>},
        MainRoute::SettingsRoot | MainRoute::Settings => html! {<Switch<SettingsRoute> render={switch_settings}/>},
        MainRoute::SearchRoot | MainRoute::Search => {
            html!{<Switch<SearchRoute> render={switch_search}/>}
        },
        MainRoute::NotFound => html! {<div><Navbar/><NotFound/></div>},
    }
}
fn switch_settings(route: SettingsRoute) -> Html {
    
    match route {
        SettingsRoute::GeneralSettings => html! {""},
        SettingsRoute::Wallets => html! {""},
        SettingsRoute::MyContracts => html! {""},
        SettingsRoute::Privacy => html! {""},

    }
}

fn switch_search(route: SearchRoute) -> Html {
    match route {
        SearchRoute::Hashtag {search} => html! {format!("")},
        SearchRoute::Profile {profile} => html! {format!("")},
        SearchRoute::Info {info} => html! {format!("")},
        SearchRoute::Trending => html! {},

    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();

        let go_home_button = {
        let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
    html! {
        <button{onclick}>{"Home"}</button>
        }
    };
        let explore_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&SearchRoute::Trending));
        html! {
            <button{onclick}>{"Explore"}</button>
        }
        };
    let settings_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&SettingsRoute::GeneralSettings));
        html! {
            <button{onclick}>{"settings"}</button>
        }
        };
    let account_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::CreateAccount));
        html! {
            <button{onclick}>{"Account"}</button>
        }
    };
                
                                html! {
                                    <div id="navigation">
                                    <div id="navbar">
                                    {go_home_button}
                                    {explore_button}
                                    {settings_button}
                                    {account_button}
                                    </div>
                                    <div id="searchspace">
                                    <input id="searchbar" type="text"/>
                                    </div>
                                    </div>
                                }
    }

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
<Switch<MainRoute> render={switch_main}/>
        </BrowserRouter>
    }
}
//profile
//manage_data
//about
//
//wallets
//contracts
//posts.
//hashtags
//feeds
//recomended 
//trending
//
//