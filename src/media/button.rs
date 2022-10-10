use yew::prelude::*;


#[derive(PartialEq, Clone, Properties)]
pub struct ButtonProps{
    pub text: String,
    pub link: String,
    pub download: String,
    pub class: String,
 }

pub struct Button{
    pub text: String,
    pub link: String,
    pub download: String
}

impl Component for Button{
    type Properties = ButtonProps;
    type Message = ();
     
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: ctx.props().text.clone(),
            link: ctx.props().link.clone(),
            download: ctx.props().download.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.download.is_empty(){
            html! (
                <a href={self.link.clone()} class={&ctx.props().class} download={self.download.clone()}>{self.text.clone()}{" •"}</a>
            )
        }else{
            html! (
                <a href={self.link.clone()} class={&ctx.props().class}>{self.text.clone()}{" •"}</a>
            )
        }
    }
}