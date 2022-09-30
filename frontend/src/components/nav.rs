use yew::{function_component, html};

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <>
        <nav class="navbar dflex-justify-center">
            <div class="dflex dflex-col">
                <div class="bold font-lg">{{"Visual Novel Engine"}}</div>
                <div>{{"Made with Rust!"}}</div>
            </div>
            <div class="dflex dflex-row dflex-gap-md">
                <a href="http://127.0.0.1:8000/">{{"Home"}}</a>
                <a href="https://github.com/grostaco">{{"GitHub"}}</a>
                <a href="https://grostaco.herokuapp.com/">{{"About Me"}}</a>
                <a href="https://github.com/grostaco/dVN">{{"This site's code"}}</a>
            </div>
        </nav>
        <div class="divider"></div>
        </>
    }
}
