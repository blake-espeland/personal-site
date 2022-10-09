use yew::prelude::*;

pub struct ProjContainer{
    pub rendered: bool
}

#[derive(Properties, PartialEq)]
pub struct ProjContainerProps {
    pub show: bool
}

pub enum Msg{
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
            rendered: ctx.props().show
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.rendered = ctx.props().show;
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            if !self.rendered{}
            else{
            <div class="project-container">
                <h1>{"Projects"}</h1>
                <button class="button" onclick={ctx.link().callback(|_| Msg::SprayerMods)}>{"Sprayer Mods"}</button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::ModelForge)}>{"ModelForge"}</button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::RNN)}>{"r_nn"}</button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::CurveEstimation)}>{"Curve Estimation"}</button>
            </div>
            }
        )
    }
}