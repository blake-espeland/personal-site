use yew::prelude::*;
use crate::Hovered;

#[derive(PartialEq, Clone, Properties)]
pub struct ListItemProps {
    #[prop_or_default]
    pub hide: bool,
    pub on_hover: Callback<Hovered>,
    pub name: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct ListItem{
    pub content: Html,
}

impl Component for ListItem{
    type Properties = ListItemProps;
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        ListItem {
            content: html! (
                <div class="list-item">{&ctx.props().name}</div>
            )
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouseover = {
            let name = ctx.props().name.clone();
            ctx.props().on_hover.reform(move |e: MouseEvent| {
                e.stop_propagation();
                Hovered::Item(name.clone())
            })
        };
        html!(
            <>
            <div class="list-item" {onmouseover}>
                { Self::view_details(&ctx.props().children) }
            </div>
            </>
        )
    }
}

impl ListItem {
    fn view_details(children: &Children) -> Html {
        if children.is_empty() {
            html! {}
        } else {
            html! {
                <div class="list-item-details">
                    { children.clone() }
                </div>
            }
        }
    }
}

impl Default for ListItem{
    fn default() -> Self {
        Self {
            content: Html::default()
        }
    }
}