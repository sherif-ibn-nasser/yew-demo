use yew::prelude::*;

enum Msg {
    Inc,
    Dec,
}

struct CounterComp {
    count: i64,
}

impl Component for CounterComp {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={ link.callback(|_| Msg::Inc) }>{ "Inc" }</button>
                <button id="dec-btn" onclick={ link.callback(|_| Msg::Dec) }>{ "Dec" }</button>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Inc => {
                self.count += 1;
                true
            }
            Msg::Dec => {
                self.count -= 1;
                true
            }
        }
    }
}

fn main() {
    yew::Renderer::<CounterComp>::new().render();
}
