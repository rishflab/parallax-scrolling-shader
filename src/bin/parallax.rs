extern crate erlking;

use erlking::app::App;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let app = futures::executor::block_on(App::new("erlking", &event_loop));
    app.run(event_loop);
}
