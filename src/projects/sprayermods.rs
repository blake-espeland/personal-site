use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub show: bool
}

pub struct SprayerMods{
    pub render: bool
}

impl Component for SprayerMods{
    type Properties = Props;
    type Message = ();

    
}