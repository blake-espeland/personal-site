mod background;
mod media;
mod projects;

use media::about::AboutSection;
use media::title::Title;

use projects::container::ProjContainer;

use background::settings::Settings;
use background::simulation::Simulation;

use wasm_logger;
use yew::{html, Component, Context, Html};

pub enum Msg {
    ChangeSettings(Settings),
    ResetSettings,
    RestartSimulation,
    TogglePause,
    ToggleAbout,
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
            render_about: true,
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
                    <a onclick={ctx.link().callback(|_| Msg::ToggleAbout)} class="side button">{self.get_proj_btn_txt()}</a>
                    <a class="button" href="https://linkedin.com/in/blake-espeland/">{self.get_linkedin_icon()}</a>
                    <a class="button" href="https://github.com/blake-espeland/">{self.get_github_icon()}</a>
                    <a class="button" download="Blake_Espeland_Resume.pdf" href="resources/Blake_Espeland_Resume.pdf" title="Download Resume">
                        {self.get_resume_icon()}
                    </a>
                    <a onclick={ctx.link().callback(|_| Msg::TogglePause)} class="button">{self.get_pause_play()}</a>
                </div>

                <ProjContainer show={!self.render_about}/>
                <AboutSection show={self.render_about}/>
            </>
        }
    }
}
impl Model {
    fn get_proj_btn_txt(&self) -> &str {
        if self.render_about {
            "Projects"
        } else {
            "About"
        }
    }

    fn get_resume_icon(&self) -> Html {
        html!(<i class="fa-solid fa-file-arrow-down icon"></i>)
    }

    fn get_pause_play(&self) -> Html {
        if self.paused {
            html!(<i class="fa fa-play icon" aria-hidden="true"></i>)
        } else {
            html!(<i class="fa fa-pause icon" aria-hidden="true"></i>)
        }
    }

    fn get_linkedin_icon(&self) -> Html {
        html!(<i class="fa-brands fa-linkedin icon"></i>)
    }

    fn get_github_icon(&self) -> Html {
        html!(<i class="fa-brands fa-github icon"></i>)
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}
