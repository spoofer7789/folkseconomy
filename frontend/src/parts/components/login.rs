use crate::parts::fragments::input::*;
use crate::parts::utils::router::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo_net::http::{Request, Method};
use wasm_bindgen_futures::spawn_local;
use yew_router::{prelude::*, navigator};
use core::option::Option::*;
pub enum Msg {
    HoverIndex(usize),
    Submit,
}
pub struct LoginPage {
    refs: Vec<NodeRef>,
    focus_index: usize,
    errors: Vec<String>,
}//
// Create
impl LoginPage {
    fn apply_focus(&self) {
        if let Some(input) = self.refs[self.focus_index].cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }
}
impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            focus_index: 0,
            refs: vec![NodeRef::default(), NodeRef::default()],
            errors: vec!["missing field".to_string(), "Error".to_string()],
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HoverIndex(index) => {
                self.focus_index = index;
                self.apply_focus();
                return false;
            }
            Msg::Submit => {
                 let username = &self.refs[0];
                 let password =&self.refs[1];
                 let password_value = password.cast::<HtmlInputElement>().unwrap().value();
                 let username_value = username.cast::<HtmlInputElement>().unwrap().value();
                self.errors[0].clear();
                self.errors[1].clear();
                if username_value == "" {
                    self.errors[0].to_string();
                    self.apply_focus();
                    return false;
                }
                if password_value == "" {
                    self.errors[1].to_string();
                    self.apply_focus();
                    return false;
                }
                let request = Request::new("http://localhost:3000/api").method(Method::POST)
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"email":"{}","password":"{}"}}"#, username_value, password_value));
           return true;
          }
    }

    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let navigator = ctx.link().navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&MainRoute::Web3Login));
    let web3signin = html!{
        <a {onclick}>{" For Web3 Login Click here"}</a>
    };
        html! {
        <div class={classes!("main", "form")}>
                <div id="left-pane">
                    <div>
                        <h1>{"Login"}</h1>
                        <div class="input-container">
                           <InputComponent
                           input_ref={&self.refs[0]}
                           on_hover={ctx.link().callback(|_| Msg::HoverIndex(0))}
                            placeholder="username" 
                            />
                        </div>
                        <div class="input-container">
                            <InputComponent
                                input_ref={&self.refs[1]}
                                on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                                placeholder="password"
                            />
                        </div>
                        <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Login"}</button>
                    </div>
                </div>
                <div id="right-pane">
                  <div>
                    <ul>
                    {web3signin}
                    </ul>
                  </div>
                </div>
            </div>
        }
    }

}