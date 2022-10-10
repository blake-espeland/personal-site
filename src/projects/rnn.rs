use yew::prelude::*;


pub struct RNN;

impl Component for RNN{
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
                <h1>{"Rust NN"}</h1>
                <h2>{"A neural network API written in Rust"}</h2>
                <p>{"This project is currently in progress as I continue learning the ins and outs of the Rust programming
                     language. The end goal is to create an API with similar syntax and tooling to PyTorch. I have no 
                     intentions of this being used by anyone, as it's more of a learning experience."}</p>
                <p>{"You can check it out "}
                    <a href={"https://github.com/blake-espeland/nn.rs"} class={"inner-p"}>{"here."}</a>
                </p>
            </>
        )
    }
}