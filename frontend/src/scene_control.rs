//use log::info;
use yew::{Callback, Properties, Component, Context, html, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub maxlen: usize,
    pub onclick: Callback<usize>,
    #[prop_or_default]
    pub index: Option<usize>
}

pub enum Msg {
    Prev,
    Next,
}

pub struct ButtonInput {
    index: usize 
}

impl Component for ButtonInput {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { index: ctx.props().index.unwrap_or(0) }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Prev => { self.index = self.index.saturating_sub(1) },
            Msg::Next => { self.index = (self.index + 1).min(ctx.props().maxlen - 1) }
        }

        ctx.props().onclick.emit(self.index);

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        html! {
            <>
            <button class="btn" onclick={link.callback(|_| Msg::Prev)}>{"Prev"}</button>
            <button class="btn" onclick={link.callback(|_| Msg::Next)}>{"Next"}</button>
            </>
        }
    }
}