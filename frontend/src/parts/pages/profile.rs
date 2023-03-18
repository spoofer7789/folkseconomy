use yew::prelude::*;

// Define a struct to hold the properties
#[derive(Properties, Clone, PartialEq)]
pub struct ProfilePageProps {
    pub public_key: String,
    pub username: String,
    pub files: Vec<String>,
}

// Update the function component to accept the new properties
#[function_component(ProfilePage)]
pub fn profile_page(props: &ProfilePageProps) -> Html {
    html! {
        // Use the properties inside the component, for example:
        <div>
            <h1>{ &props.username }</h1>
            <p>{ "Ethereum Public Key: " }{ &props.public_key }</p>
            <ul>
                { for props.files.iter().map(|file| html!{ <li>{ file.clone() }</li> }) }
            </ul>
        </div>
    }
}
