use seed::{prelude::*, *};

struct Model {
    pub counter: i32,
}

impl Default for Model {
    // In this case, we could derive `Default` instead.
    fn default() -> Self {
        Self { counter: 0 }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let button_class = class!["bg-gray-400", "px-8", "py-4"];
    div![
        class![
            "flex",
            "flex-col",
            "justify-center",
            "items-center",
            "h-screen",
            "text-gray-600"
        ],
        button![
            button_class,
            simple_ev(Ev::Click, Msg::Increment),
            format!("Click Me!")
        ],
        div![
            class!["w-56", "text-center", "mt-2"],
            format!("Click {} times", model.counter)
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
