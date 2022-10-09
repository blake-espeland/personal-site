use gloo::console::log;
use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AboutSectionProps{
    pub show: bool
}

pub struct AboutSection{
    pub rendered: bool
}


impl Component for AboutSection{
    type Message = ();
    type Properties = AboutSectionProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            rendered: ctx.props().show
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        log!("Updated about: {}", ctx.props().show);
        self.rendered = ctx.props().show;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
        if !self.rendered {}
        else{
            <div class={"about-container"}>
                <p class={"about-text"}>
                    {"I am a passionate developer and engineer with interests at every level of abstraction. I enjoy low-level, embedded programming; large-scale 
                        application development; visual and human-based design; and machine learning."}
                </p>
                <p class={"about-text"}>{"Other topics of interest include mathematics, physics, and art."}</p>
                <p class={"about-text"}>{"In my free time, I enjoy being outside. But if it's too inclement, I'll be inside reading or playing video games."}</p>
            </div>
        }
        )
    }
}