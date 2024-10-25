use urlencoding::decode;
mod extract;
fn main() {
    use tao::event_loop::EventLoop;
    let event_loop = EventLoop::new();
    event_loop.run(move |event, _, _control_flow| {
        match event {
            tao::event::Event::Opened { urls } => {
                let clean_path = decode(urls.get(0).expect("Path not provided").as_str().replace("file://", "").as_str()).unwrap().to_string();
                let input_path = std::path::Path::new(&clean_path);
                extract::extract_recourse(input_path);
            }
            _ => {}
        }
    });    
}




