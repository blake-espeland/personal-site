use super::list::{List, Msg as ListMsg};
use crate::{Hovered, WeakComponentLink};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ListHeaderProps {
    pub on_hover: Callback<Hovered>,
    pub text: String,
    pub list_link: WeakComponentLink<List>,
}

pub struct ListHeader;

impl Component for ListHeader {
    type Message = ();
    type Properties = ListHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list_link = ctx.props().list_link.borrow().clone().unwrap();
        let onmouseover = ctx.props().on_hover.reform(|e: MouseEvent| {
            e.stop_propagation();
            Hovered::Header
        });

        html! {
            <div
                class="list-header"
                {onmouseover}
                onclick={list_link.callback(|_| ListMsg::HeaderClick)}
            >
                { &ctx.props().text }
            </div>
        }
    }
}