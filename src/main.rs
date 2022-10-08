mod background;
mod media;

use background::math::Vector2D;
use media::button::Button;

use background::settings::Settings;
use background::simulation::Simulation;

use wasm_bindgen::JsCast;
use yew::functional::use_state_eq;
use yew::ContextProvider;
use yew::{html, Component, Context, Html};

use web_sys::Window;
pub enum Msg {
    ChangeSettings(Settings),
    ResetSettings,
    RestartSimulation,
    TogglePause,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameProps {
    x: f64,
    y: f64,
}

pub struct Model {
    settings: Settings,
    generation: usize,
    paused: bool,
    window_size: Vector2D,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let w = web_sys::window().expect("Couldn't get window element.");
        Self {
            settings: Settings::load(),
            generation: 0,
            paused: false,
            window_size: Vector2D {
                x: match w.inner_width() {
                    Ok(v) => v.as_f64().expect("Couldn't get window width."),
                    Err(_) => 1080.0,
                },
                y: match w.inner_height() {
                    Ok(v) => v.as_f64().expect("Couldn't get window height."),
                    Err(_) => 1920.0,
                },
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::ChangeSettings(settings) => {
                self.settings = settings;
                self.settings.store();
                true
            }
            Msg::ResetSettings => {
                self.settings = Settings::default();
                Settings::remove();
                true
            }
            Msg::RestartSimulation => {
                self.generation = self.generation.wrapping_add(1);
                true
            }
            Msg::TogglePause => {
                self.paused = !self.paused;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let w = web_sys::window().expect("Couldn't get window element.");

        html! {
            <>
                <h1 class="title">{ "Blake Espeland" }</h1>
                <Simulation settings={self.settings.clone()} generation={self.generation} paused={self.paused}/>
                <div class="button-container">
                    <Button text={"GitHub"} class={"button"} download={""} link={"https://github.com/blake-espeland/"}/>
                    <Button text={"LinkedIn"} class={"button"} download={""} link={"https://linkedin.com/in/blake-espeland/"}/>
                    <Button text={"Resume"} class={"button"} download={"Blake_Espeland_Resume.docx"} link={"resources/Blake_Espeland_Resume.docx"}/>
                    <Button text={"Projects"} class={"button"} download={""} link={""}/>
                </div>
            </>
        }
    }
}
impl Model {}

fn main() {
    yew::start_app::<Model>();
}
