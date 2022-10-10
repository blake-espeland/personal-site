use yew::prelude::*;

use crate::Model;

use super::curve_est::CurveEstimation;
use super::model_forge::ModelForge;
use super::rnn::RNN;
use super::sprayer_mods::SprayerMods;

pub struct ProjContainer {
    pub render: bool,
    proj_highlight: Msg,
}

#[derive(Properties, PartialEq)]
pub struct ProjContainerProps {
    pub show: bool,
}

#[derive(PartialEq)]
pub enum Msg {
    None,
    SprayerMods,
    ModelForge,
    RNN,
    CurveEstimation,
}

impl Component for ProjContainer {
    type Properties = ProjContainerProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            render: ctx.props().show,
            proj_highlight: Msg::None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _p: &ProjContainerProps) -> bool {
        self.render = !self.render;
        self.proj_highlight = Msg::None;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SprayerMods => {
                self.proj_highlight = Msg::SprayerMods;
            }
            Msg::ModelForge => {
                self.proj_highlight = Msg::ModelForge;
            }
            Msg::RNN => {
                self.proj_highlight = Msg::RNN;
            }
            Msg::CurveEstimation => {
                self.proj_highlight = Msg::CurveEstimation;
            }
            Msg::None => {
                self.proj_highlight = Msg::None;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.render {
            return html!();
        }

        match self.proj_highlight {
            Msg::None => {
                html!(
                <div class="o-project-container">
                    <div class="project-container">
                        <h1>{"Projects"}</h1>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::SprayerMods)}>{"Sprayer Mods"}</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::ModelForge)}>{"ModelForge"}</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::RNN)}>{"Rust NN"}</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::CurveEstimation)}>{"Curve Estimation"}</button>
                    </div>
                </div>
                )
            }
            _ => {
                html!(
                    <div class="o-project-container">
                        <button class="button" onclick={ctx.link().callback(|_| Msg::None)}>{"back"}</button>
                        <div class="project-container">
                            {self.get_inner_html()}
                        </div>
                    </div>
                )
            }
        }
    }
}

impl ProjContainer {
    fn get_inner_html(&self) -> Html {
        match self.proj_highlight {
            Msg::SprayerMods => {
                html!(<SprayerMods/>)
            }
            Msg::ModelForge => {
                html!(<ModelForge/>)
            }
            Msg::RNN => {
                html!(<RNN/>)
            }
            Msg::CurveEstimation => {
                html!(<CurveEstimation/>)
            }
            _ => {
                html!()
            }
        }
    }
}
