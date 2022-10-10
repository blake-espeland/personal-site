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
                <h1>{"Curve Estimation"}</h1>
                <h2>{"A final project for CS:5430 - Machine Learning."}</h2>
                <p>{"In this project, my partner, Leodona Odole, and I created an application that takes in an
                     image of a row crop field as input, and outputs the coefficients of a fitted polynomial. The polynomial
                     is an estimate of the field's curvature, which can be used in robotics scheduling tasks."}</p>
                <img src="resources/cotton-orig.jpg"/>
                <p>{"Original"}</p>
                <img src="resources/cotton-anno.jpg"/>
                <p>{"Polynomial Overlay"}</p>
                <p>{"In short, the program works by first sending the image through the ModelForge algorithm, removing weeds in the image.
                     Then, it gets rid of the perspective shift from the image (i.e. turning it into an overhead image).
                     Next, it performs a green mask to select the green pixels. The best row is selected (closed to center, and most green pixels),
                     and the polynomial is fit to those datapoints."}</p>
                <p>{"The results from this project were used by Sprayer Mods for accurately determining how much
                     to delay or expidite spray signals."}</p>
            </>

        )
    }
}