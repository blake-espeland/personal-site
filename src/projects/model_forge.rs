use yew::prelude::*;


pub struct ModelForge;

impl Component for ModelForge{
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
                <h1><a class="inner-p" href={"https://github.com/Sprayer-Mods/ModelForge"}>{"Model Forge"}</a></h1>

                <p>
                    {"A part of the Sprayer Mods project. In order to detect weeds, 
                    we needed an accurate and light-weight computer vision model.
                    I decided to start with the "}
                    <a class="inner-p" href={"https://github.com/ultralytics/yolov5"}>{"Ultralytics YOLOv5"}</a>
                    {" PyTorch repository. Atop that, I implemented "}
                    <a class="inner-p" href={"https://arxiv.org/abs/2207.02696"}>{"YOLOv7"}</a>
                    {" and vision transformers. I was able to achieve 92 mAP on a dataset of corn images."}
                </p>
                
                <img src="resources/yolov7-res1.png"/>
                <br/>
                <img src="resources/00003234.jpg"/>
                <img src="resources/00003234pred.jpg"/>

                <h2>{"Added Features:"}</h2>
                <ul>
                    <li>{"WandB visualization and monitoring"}</li>
                    <li>{"Video + sequential data loading"}</li>
                    <li>{"Video + sequential training"}</li>
                    <li>{"OpenVINO quantization"}</li>
                    <li>{"Time Shift Module"}</li>
                </ul>
            </>

        )
    }
}