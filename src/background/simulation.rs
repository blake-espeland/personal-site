use super::boid::Boid;
use super::math::Vector2D;
use super::settings::Settings;
use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

use web_sys::Window;

#[derive(Debug)]
pub enum Msg {
    Tick
}



#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub settings: Settings,
    #[prop_or_default]
    pub generation: usize,
    #[prop_or_default]
    pub paused: bool,
    
}

#[derive(Debug)]
pub struct Simulation {
    boids: Vec<Boid>,
    interval: Interval,
    pub win: Window,
}

pub fn get_window_size(w: &Window) -> Vector2D{
    Vector2D {
        x: match w.inner_width() {
            Ok(v) => v.as_f64().expect("Couldn't get window width."),
            Err(_) => 1080.0,
        },
        y: match w.inner_height() {
            Ok(v) => v.as_f64().expect("Couldn't get window height."),
            Err(_) => 1920.0,
        },
    }
}

impl Component for Simulation {
    type Message = Msg;
    type Properties = Props;



    fn create(ctx: &Context<Self>) -> Self {
        let w = web_sys::window().expect("Couldn't get window element.");

        let bounds: Vector2D = get_window_size(&w);
        let settings = &ctx.props().settings;
        let boids = (0..settings.boids)
            .map(|_| Boid::new_random(settings, &bounds))
            .collect();

        let interval = {
            let link = ctx.link().clone();
            Interval::new(settings.tick_interval_ms as u32, move || {
                link.send_message(Msg::Tick)
            })
        };
        Self { boids, interval, win: w }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                let Props {
                    ref settings,
                    paused,
                    ..
                } = *ctx.props();



                if paused {
                    false
                } else {
                    Boid::update_all(settings, &mut self.boids, &get_window_size(&self.win));
                    true
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.boids.clear();
        let bounds: Vector2D = get_window_size(&self.win);
        let settings = &ctx.props().settings;
        self.boids
            .resize_with(settings.boids, || Boid::new_random(settings, &bounds));

        // as soon as the previous task is dropped it is cancelled.
        // We don't need to worry about manually stopping it.
        self.interval = {
            let link = ctx.link().clone();
            Interval::new(settings.tick_interval_ms as u32, move || {
                link.send_message(Msg::Tick)
            })
        };

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let size = get_window_size(&self.win);
        let view_box = format!("0 0 {} {}", size.x, size.y);

        html! {
            <svg class="simulation-window" viewBox={view_box}>
                { for self.boids.iter().map(Boid::render) }
            </svg>
        }
    }


}
