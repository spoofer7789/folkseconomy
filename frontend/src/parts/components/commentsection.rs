use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GCS {
    pub user: String,
    pub comment: String,
}


#[function_component(CommentSection)]
pub fn comment_section() -> Html {
    
    //create a function that returns previous saved 
    // comments.
    html! {

    }
}