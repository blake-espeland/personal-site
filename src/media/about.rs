use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AboutSectionProps{
    pub show: bool
}

pub struct AboutSection{
    pub render: bool
}


impl Component for AboutSection{
    type Message = ();
    type Properties = AboutSectionProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            render: ctx.props().show
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _p: &AboutSectionProps) -> bool {
        self.render = !self.render;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
        if !self.render {}
        else{
            <div class={"project-container"}>
                <h1>{"About"}</h1>
                <p class={"about-text"}>
                    {"Hello! I'm a software engineer with interests at every level of abstraction. I enjoy low-level, embedded programming; large-scale 
                        application development; visual and human-based design; and machine learning."}
                </p>
                <p class={"about-text"}>{"Other topics of interest include math, physics, history, philosphy, and art."}</p>
                <p class={"about-text"}>{"In my free time, I enjoy being outside. But if it's too inclement, I'll be inside reading or playing video games."}</p>
                <p class={"about-text"}>{"Feel free to check out my projects highlighted on this page. For more, check out my LinkedIn or GitHub."}</p>
                <p class={"about-text"}>{"This website was made using the Yew frontend framework for the Rust programming language."}</p>
                <small>{"2022 - Made with "}<i class="fa-regular fa-heart"></i>{" and Rust "}</small>
            </div>
        }
        )
    }
}