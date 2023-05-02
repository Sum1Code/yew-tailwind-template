mod app;
mod tailwindparser;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
