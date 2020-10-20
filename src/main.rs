use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let position = app.mouse.position();
    draw.background().color(LIGHTBLUE);
    draw.ellipse().color(RED).w(100.0).h(100.0).xy(position);
    draw.to_frame(app, &frame).unwrap();
}
