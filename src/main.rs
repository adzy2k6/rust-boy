extern crate sdl2;

fn main() {
    let result = sdl2::init();
    let sdl = result.unwrap();

    //Get the video subsystem and create a window.
    let video = sdl.video().unwrap();
    let window_builder = video.window("Hello Window", 100, 100);
    let window = window_builder.build().unwrap();

    //Get a timer and delay for 2s.
    let mut timer = sdl.timer().unwrap();
    timer.delay(2000);
}
