use yew::prelude::*;

pub struct SprayerMods;

impl Component for SprayerMods{
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
                <h1 class="project-h1">{"Sprayer Mods"}</h1>
                <p class="project-p">{"lorem ipsum lol"}</p>
                <p class="project-p">{"lorem ipsum lol"}</p>
            </>

        )
    }
}