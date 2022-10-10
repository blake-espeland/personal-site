use yew::prelude::*;



pub struct ProjContainer{
    pub render: bool,
    proj_highlight: Msg
}

#[derive(Properties, PartialEq)]
pub struct ProjContainerProps {
    pub show: bool,
}

#[derive(PartialEq)]
pub enum Msg{
    None,
    SprayerMods,
    ModelForge,
    RNN,
    CurveEstimation
}

impl Component for ProjContainer{
    type Properties = ProjContainerProps;
    type Message = Msg;
    
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            render: ctx.props().show,
            proj_highlight: Msg::None
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _p: &ProjContainerProps) -> bool {
        self.render = !self.render;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SprayerMods => {
                self.proj_highlight = Msg::SprayerMods;
            },
            Msg::ModelForge => {
                self.proj_highlight = Msg::ModelForge;
            },
            Msg::RNN => {
                self.proj_highlight = Msg::RNN;
            },
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
        html!(
            if !self.render{}
            else{
            <div class="project-container">
                <h2 class="project-h2">{"Projects"}</h2>
                <button class="button" onclick={ctx.link().callback(|_| Msg::SprayerMods)}>{"• Sprayer Mods"}</button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::ModelForge)}>{"• ModelForge"}</button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::RNN)}>{"• r_nn"}</button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::CurveEstimation)}>{"• Curve Estimation"}</button>
            </div>
            }
        )
    }
}