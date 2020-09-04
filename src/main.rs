use nannou::prelude::*;

fn main() {
    println!("\nWelcome to the dark place!\n");
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    stream: nannou_audio::Stream<Audio>,
}


struct Audio {}

fn model(_app: &App) -> Model {
    println!("Setting things up...");
    let host = nannou_audio::Host::new();
    let audio_model = Audio {};
    let input_stream = host.new_input_stream(audio_model)
        .capture(audio)
        .build()
        .unwrap();

    Model { stream: input_stream }
}
fn audio(audio: &mut Audio, buffer: & nannou_audio::buffer::Buffer) {
    for frame in buffer.frames() {
        print!("{:?} ", frame)
    }
    print!("\n")
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
}