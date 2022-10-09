use yew::prelude::*;

#[derive(Properties, PartialEq, PartialOrd)]
pub struct TitleProps{
    pub show: bool
}

pub struct Title;


impl Component for Title{
    type Properties = TitleProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.props().show
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
            <div class={"title-container"}>
                <h1 class={"title"}>{ "Blake Espeland" }</h1>
                <p class={"under-title"}>{"Salus populi suprema lex esto"}</p>
            </div>
        )
    }
}