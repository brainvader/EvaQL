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
    button![
        simple_ev(Ev::Click, Msg::Increment),
        format!("Hello, World × {}", model.counter)
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
