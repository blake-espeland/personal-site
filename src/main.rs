mod background;
mod list;

use list::list::List;
use list::header::ListHeader;
use list::item::ListItem;

use background::settings::Settings;
use background::simulation::Simulation;

use yew::html::Scope;
use yew::{html, html_nested, Component, Context, Html, MouseEvent};
use yew::html::ImplicitClone;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct WeakComponentLink<COMP: Component>(Rc<RefCell<Option<Scope<COMP>>>>);

impl<COMP: Component> Clone for WeakComponentLink<COMP> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<COMP: Component> ImplicitClone for WeakComponentLink<COMP> {}

impl<COMP: Component> Default for WeakComponentLink<COMP> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

impl<COMP: Component> Deref for WeakComponentLink<COMP> {
    type Target = Rc<RefCell<Option<Scope<COMP>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<COMP: Component> PartialEq for WeakComponentLink<COMP> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug)]
pub enum Hovered {
    Header,
    Item(String),
    List,
    None,
}

pub enum Msg {
    ChangeSettings(Settings),
    ResetSettings,
    RestartSimulation,
    TogglePause,
    Hover(Hovered)
}

pub struct Model {
    settings: Settings,
    generation: usize,
    paused: bool,
    hovered: Hovered,
    list_link: WeakComponentLink<List>,
    sub_list_link: WeakComponentLink<List>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            settings: Settings::load(),
            generation: 0,
            paused: false,
            hovered: Hovered::None,
            list_link: WeakComponentLink::default(),
            sub_list_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::ChangeSettings(settings) => {
                self.settings = settings;
                self.settings.store();
                true
            }
            Msg::ResetSettings => {
                self.settings = Settings::default();
                Settings::remove();
                true
            }
            Msg::RestartSimulation => {
                self.generation = self.generation.wrapping_add(1);
                true
            }
            Msg::TogglePause => {
                self.paused = !self.paused;
                true
            }
            Msg::Hover(h) => {
                self.hovered = h;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            ref settings,
            generation,
            paused,
            ..
        } = *self;

        let on_hover = &ctx.link().callback(Msg::Hover);
        let _onmouseover = &ctx.link().callback(|_: MouseEvent| Msg::Hover(Hovered::None));
        let onmouseoversublist = &ctx.link().callback(|e: MouseEvent| {
            e.stop_propagation();
            Msg::Hover(Hovered::List)
        });
        let list_link = &self.list_link;
        let sub_list_link = &self.sub_list_link;

        // note the use of `html_nested!` instead of `html!`.
        let letters = ('A'..='C')
            .map(|letter| html_nested! { <ListItem name={letter.to_string()} {on_hover} /> });

        html! {
            <>
                <h1 class="title">{ "Blake Espeland" }</h1>
                <Simulation settings={settings.clone()} {generation} {paused} />
                { self.view_panel(ctx.link()) }
                <List {on_hover} weak_link={list_link}>
                    <ListHeader text="Calling all Rusties!" {on_hover} {list_link} />
                    <ListItem name="Rustin" {on_hover} />
                    <ListItem hide=true name="Rustaroo" {on_hover} />
                    <ListItem name="Rustifer" {on_hover}>
                        <div class="sublist" onmouseover={onmouseoversublist}>{ "Sublist!" }</div>
                        <List {on_hover} weak_link={sub_list_link}>
                            <ListHeader text="Sub Rusties!" {on_hover} list_link={sub_list_link}/>
                            <ListItem hide=true name="Hidden Sub" {on_hover} />
                            { for letters }
                        </List>
                    </ListItem>
                </List>
            </>
        }
    }
}
impl Model {
    fn view_panel(&self, _link: &Scope<Self>) -> Html {
        html! {

        }
    }
}

fn main() {
    yew::start_app::<Model>();
}