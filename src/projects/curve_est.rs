use yew::prelude::*;


pub struct CurveEstimation;

impl Component for CurveEstimation{
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
                <h1 class="project-h1">{"Curve Estimation"}</h1>
                <p class="project-p">{"lorem ipsum lol"}</p>
                <p class="project-p">{"lorem ipsum lol"}</p>
            </>

        )
    }
}