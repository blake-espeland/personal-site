mod background;
mod media;
mod projects;

use media::button::Button;
use media::about::AboutSection;
use media::title::Title;

use projects::container::ProjContainer;

use background::settings::Settings;
use background::simulation::Simulation;

use yew::{html, Component, Context, Html};
use wasm_logger;

pub enum Msg {
    ChangeSettings(Settings),
    ResetSettings,
    RestartSimulation,
    TogglePause,
    ToggleAbout
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
    render_about: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            settings: Settings::load(),
            generation: 0,
            paused: false,
            render_about: true
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
            Msg::ToggleAbout => {
                self.render_about = !self.render_about;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Title show={true}/>
                <Simulation settings={self.settings.clone()} generation={self.generation} paused={self.paused}/>
                <div class="button-container">
                    <Button text={"GitHub"} class={"button"} download={""} link={"https://github.com/blake-espeland/"}/>
                    <Button text={"LinkedIn"} class={"button"} download={""} link={"https://linkedin.com/in/blake-espeland/"}/>
                    <Button text={"Resume"} class={"button"} download={"Blake_Espeland_Resume.docx"} link={"resources/Blake_Espeland_Resume.docx"}/>
                    <button onclick={ctx.link().callback(|_| Msg::ToggleAbout)} class="button">{self.get_proj_btn_txt()}</button>
                </div>
                <ProjContainer show={!self.render_about}/>

                <AboutSection show={self.render_about}/>
            </>
        }
    }
}
impl Model {
    fn get_proj_btn_txt(&self) -> &str{
        if self.render_about{
            "Projects"
        }else{
            "About"
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
