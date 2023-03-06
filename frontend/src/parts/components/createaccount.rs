use gloo_net::http::Request;
use crate::parts::fragments::input::*;
use crate::parts::utils::router::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
pub enum Msg {
    HoverIndex(usize),
    Submit,
}

pub struct CreateAccount {
    refs: Vec<NodeRef>,
    focus_index: usize,
    email_error: String,
    password_error: String,
    email: String,
    password: String,
}
impl CreateAccount {
    fn apply_focus(&self) {
        if let Some(input) = self.refs[self.focus_index].cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }
}
impl Component for CreateAccount {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus_index: 0,
            refs: vec![NodeRef::default(), NodeRef::default()],
            email_error: "".to_string(),
            password_error: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Msg::HoverIndex(index) = msg {
            self.focus_index = index;
            self.apply_focus();
            false
        } else {
                        let email = &self.refs[0];
                        let password = &self.refs[1];
                        let email_value = email.cast::<HtmlInputElement>().unwrap().value();
                        let password_value = password.cast::<HtmlInputElement>().unwrap().value();
                
                        self.email_error.clear();
                        self.password_error.clear();

                        if !(email_value.contains('@') && email_value.contains('.')) {
                            self.email_error.push_str("Invalid email.")
                        }
                        if password_value.len() < 8 {
                            self.password_error
                                .push_str("Password must be at least 8 characters long.")
                        }
                        self.email = email_value;
                        self.password = password_value;



                        let accountdata =(email,password);
                    //create an async function

                    let createaccount = Request::post("/api {}").send();

                    //build a rust server that will
                        true
                        //make a statement that returns the username and passwords
                        //If the user using a post request
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
                        <h1>{"Create your account"}</h1>
                        <div class="input-container">
                            <label>{ "Email" }</label>
                            <input
                                type="text"
                                ref={&self.refs[0]}
                                class="input-element"
                                onmouseover={ctx.link().callback(|_| Msg::HoverIndex(0))}
                                placeholder="abcd@xyz.com"
                            />
                            <div class="error">{self.email_error.clone()}</div>
                        </div>
                        <div class="input-container">
                            <label>{ "Password" }</label>
                            <InputComponent
                                input_ref={&self.refs[1]}
                                on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                                placeholder="password"
                                
                            />
                            <div class="error">{self.password_error.clone()}</div>
                        </div>
                        <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Create"}</button>
                    </div>
                </div>
                <div id="right-pane">
                    <div>
                        <div id="graphic"></div>
                        <ul>
                        {web3signin}
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.apply_focus();
        }
    }
}