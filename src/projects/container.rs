use super::cepa::CEPA;
use super::curve_est::CurveEstimation;
use super::model_forge::ModelForge;
use super::sprayer_mods::SprayerMods;
use yew::prelude::*;

pub struct ProjContainer {
    pub render: bool,
    proj_highlight: Proj,
}

#[derive(Properties, PartialEq)]
pub struct ProjContainerProps {
    pub show: bool,
}

#[derive(PartialEq)]
pub enum Proj {
    None,
    SprayerMods,
    ModelForge,
    CEPA,
    CurveEstimation,
}

impl Component for ProjContainer {
    type Properties = ProjContainerProps;
    type Message = Proj;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            render: ctx.props().show,
            proj_highlight: Proj::None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _p: &ProjContainerProps) -> bool {
        self.render = !self.render;
        self.proj_highlight = Proj::None;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Proj::SprayerMods => {
                self.proj_highlight = Proj::SprayerMods;
            }
            Proj::ModelForge => {
                self.proj_highlight = Proj::ModelForge;
            }
            Proj::CEPA => {
                self.proj_highlight = Proj::CEPA;
            }
            Proj::CurveEstimation => {
                self.proj_highlight = Proj::CurveEstimation;
            }
            Proj::None => {
                self.proj_highlight = Proj::None;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.render {
            return html!();
        }

        match self.proj_highlight {
            Proj::None => {
                html!(
                    <div class="project-container">
                        <h1>{"Selected Projects"}</h1>
                        <a class="button" onclick={ctx.link().callback(|_| Proj::SprayerMods)}>{"Sprayer Mods"}</a>
                        <a class="button" onclick={ctx.link().callback(|_| Proj::ModelForge)}>{"ModelForge"}</a>
                        <a class="button" onclick={ctx.link().callback(|_| Proj::CEPA)}>{"CEPA"}</a>
                        <a class="button" onclick={ctx.link().callback(|_| Proj::CurveEstimation)}>{"Curve Estimation"}</a>
                    </div>
                )
            }
            _ => {
                html!(
                <div class="project-container">
                    <a class="back-button" onclick={ctx.link().callback(|_| Proj::None)}><i class="fa-sharp fa-solid fa-arrow-left"></i></a>
                    {self.get_inner_html()}
                </div>
                )
            }
        }
    }
}

impl ProjContainer {
    fn get_inner_html(&self) -> Html {
        match self.proj_highlight {
            Proj::SprayerMods => {
                html!(<SprayerMods/>)
            }
            Proj::ModelForge => {
                html!(<ModelForge/>)
            }
            Proj::CEPA => {
                html!(<CEPA/>)
            }
            Proj::CurveEstimation => {
                html!(<CurveEstimation/>)
            }
            _ => {
                html!()
            }
        }
    }
}
