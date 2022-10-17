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
                <div class="head-container">
                    <h1>{"Sprayer Mods"}</h1>
                </div>
                <h2>{"CTO October 2020 - October 2022"}</h2>
                <p>{"Sprayer Mods is a startup that is attempting to revolutionize agriculture by competing
                     with industry giants like John Deere, Raven, and Case-New Holland. We created a 
                     retrofittable spot-spraying system for agricultural herbicide sprayers. In English, 
                     that means a grower doesn't need to spray the parts of the field where weeds aren't, and they 
                     don't have to buy a new system."}</p>
                <img src="resources/early.JPG"/>
                <p>{"We started with no funding and no real industry knowledge of any kind. Yet we managed to
                     not only develop and test a system, but we were able to get a full-scale product installed
                     on a sprayer."}</p>
                <img src="resources/full-system.jpg"/>
                <p>{"I joined early on in the development of the startup, so I had a large roll in designing
                     and implementing anything technological. Along with that, I competed in pitch competitions
                     and various fundraising events, where I improved my public speaking skills and earned some free money."}</p>
                <img src="resources/pitch1.jpg"/>
                <img src="resources/pitch2.jpg"/>
                <p>{"I lead the team in creating a computer vision pipeline for data collection, annotation,
                     and training. The training and deployment software is available publicly in the "}
                     <a class="inner-p" href={"https://github.com/Sprayer-Mods/ModelForge"}>{"ModelForge"}</a>
                     {" repository. I also worked heavily on the embedded control software that took the 
                     detections from the CV model and parsed them into spray signals."}</p>
                <p>{"What started as a supplemental research project ended up being something so much more 
                     fulfilling and enriching than I could have ever imagined."}</p>
            </>
        )
    }
}