use yew::prelude::*;


#[derive(PartialEq, Clone, Properties)]
pub struct ButtonProps{
    pub text: String,
    pub link: String,
    pub show: bool,
    pub class: String
 }

pub struct Button{
    pub text: String,
    pub link: String,
    pub show: bool,
}

impl Component for Button{
    type Properties = ButtonProps;
    type Message = ();
     
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: ctx.props().text.clone(),
            link: ctx.props().link.clone(),
            show: ctx.props().show.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <a href={self.link.clone()} class={&ctx.props().class}>{self.text.clone()}</a>
        )
    }
}