use crate::tailwindparser::{parse_tailstr, TailwindStrCont};
use rand::Rng;
use yew::prelude::*;
fn randomize_colors() -> &'static str {
    let mut randomizer = rand::thread_rng();
    match randomizer.gen_range(0..6) {
        0 => "bg-red-500",
        1 => "bg-yellow-500",
        2 => "bg-purple-500",
        3 => "bg-cyan-500",
        4 => "bg-orange-500",
        5 => "bg-pink-500",
        _ => panic!("Randomizer failure"),
    }
}

#[function_component]
pub fn App() -> Html {
    let mut tailstrs = TailwindStrCont::new();
    let bg_color = use_state(|| randomize_colors());

    tailstrs.add("button_css", "bg-green-500 text-white text-3xl p-2 rounded-md font-bold transition-all duration-300 hover:text-green-500 hover:bg-white hover:scale-110");
    tailstrs.add(
        "root_div",
        format!(
            "flex flex-col justify-center items-center {} h-screen transition duration-500",
            *bg_color
        )
        .as_str(),
    );

    let onclick = {
        let bg_color = bg_color.clone();
        move |_| {
            let new_color = randomize_colors();
            bg_color.set(new_color)
        }
    };

    html!(
        <div class={classes!(parse_tailstr(tailstrs.get("root_div")))}>
           // <h1 class={classes!(parse_tailstr("text-3xl text-white font-bold pb-12".to_string()))}>{*counter}</h1>
            <button {onclick} class={classes!(parse_tailstr(tailstrs.get("button_css")))}>{"CHANGE"}</button>
    </div>
    )
}
