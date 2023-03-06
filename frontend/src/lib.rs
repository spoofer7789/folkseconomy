mod parts;
use yew::prelude::*;
use crate::parts::utils::router::Router;



    
#[function_component(App)]
pub fn app() -> Html {
html! {
    <>
<Router/>
    </>
}
}