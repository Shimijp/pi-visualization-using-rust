use nannou::prelude::*;
use std::io::{ Cursor};
use rodio::{ MixerDeviceSink};


struct Model{
    points: Vec<Point2>,
    radius: f32,
    current_theta:  f32,
    current_u : Point2,
    current_v : Point2,
    _audio_handle: MixerDeviceSink,
    _player: rodio::Player,
}
fn main() {

    nannou::app(model)
        .loop_mode(LoopMode::Rate { update_interval: std::time::Duration::from_millis(1000 / 120) })
        .update(update)
        .run();
}


fn model(_app : &App) -> Model {


    let sink_handle = rodio::DeviceSinkBuilder::open_default_sink()
        .expect("open default audio stream");

    let audio_bytes = include_bytes!("Can You Hear.mp3");

    let cursor = Cursor::new(audio_bytes);

    let player = rodio::play(&sink_handle.mixer(), cursor).unwrap();
    player.play();
    _app.new_window()
        .view(view)
        .build()
        .unwrap();
    let points = Vec::new();
    let radius = 120f32;
    let current_theta =0f32;
    Model
    {
        points,

        radius,
        current_theta,
        current_u : pt2(0f32, 0f32),
        current_v : pt2(0f32, 0f32),
        _audio_handle : sink_handle,
        _player : player
    }

}

/*fn event(_app : &App, _model : &mut Model, _event: &Event)
{

}*/
fn update(_app : &App, _model : &mut Model, _update : Update) {


    let v_x = _model.current_theta.cos();
    let v_y = _model.current_theta.sin();
    let v =_model.radius* pt2(v_x, v_y);
    let u_x = (PI * _model.current_theta).cos();
    let u_y = (PI * _model.current_theta).sin();
    let u = _model.radius * pt2(u_x, u_y) ;
    _model.current_v = v;
    _model.current_u = u;
    _model.points.push( v+ u);



    _model.current_theta += 0.03;

}

fn view(app: &App, _model : &Model, frame: Frame)
{
    let draw = app.draw();
    draw.background()
        .color(BLACK);

    draw.polyline()
        .weight(1.0)
        .points(_model.points.clone())
        .color(WHITE);
    draw.line()
        .start(pt2(0f32, 0f32))
        .end(_model.current_v)
        .color(WHITE);
    draw.line()
        .start(_model.current_v)
        .end(_model.current_u+ _model.current_v)
        .color(WHITE);

    draw.ellipse()
        .xy(pt2(0f32,0f32))
        .radius(5f32)
        .color(WHITE);

    draw.ellipse()
        .xy(_model.current_v)
        .radius(5f32)
        .no_fill()
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}