use yew::prelude::*;


pub struct CEPA;

impl Component for CEPA{
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
                <div class="head-container">
                    <h1>{"CEPA"}</h1>
                </div>
                <h2>{"Cybernetic Economic Planning Agent"}</h2>
                <h3>{"GPU-Accelerated Economic Planning"}</h3>
                <p>{"This project was inspired by my reading of Paul Cockschott and conversations with Tomas Haerdin. Both are
                    well-respected thinkers in the Cybernetic Economics space. "}</p>
                <p>{"The most fascinating aspect of cybernetics, and especially large-scale planning, is that it's so multi-modal; 
                    It requires knowledge of history, economics, politics, mathematics, computer science, large data analysis, and more.
                    Further, to handle such large amounts of data, one must utilize the latest developments in general-purpose GPUs, 
                    TPUs, DPUs, and other specialized hardware. Any serious attempt at implementing such a system would require years
                    of work and many experts in many fields. However, a simple proof of concept is enough to expose myself to all the requred
                    skillsets."}</p>
                <p>{"This project utilizes the following technologies (this is likely to change, and I'm sure the list is outdated at this point):
                     Go, to handle requests and data transfer; Julia, to provide a quick prototype of the large-scale data operations; C++ and SYCL,
                     to perform cross-platform parallel computations on nearly any kind of hardware; Rust + WASM, for the user-interfaces; and 
                     PostgeSQL, for managing the databases."}</p>
                <p>{"I chose this stack to be primarilly fun to work with and cutting-edge. If you're interested, you can check it out "}
                    <a href={"https://github.com/blake-espeland/cepa-backend"} class={"inner-p"}>{"here."}</a>
                </p>
                <p>{"Work on the frontend hasn't started yet as of the time of writing, however, it may be on my GitHub at the time you're reading this."}</p>
            </>
        )
    }
}