use std::cell::RefCell;
mod engine;
mod memodb;
mod hteapot;
use engine::Engine;
use std::env;
use hteapot::HteaPot;

const DEFAULT_PORT: u16 = 8080;

fn main() {
    
    //let args: Vec<String> = env::args().collect();
    let addr: String = String::from("0.0.0.0");
    //let port = args.get(2);
    let port = match env::var("PORT") {
            Ok(val) => val,
            Err(_) => DEFAULT_PORT.to_string(),
    };
    let teapot = HteaPot::new(&addr, port.parse().unwrap());
    let engine = RefCell::new(Engine::new());
    engine.borrow_mut().init_mock_data();
    println!("Starting server...");
    println!("Listening on {}:{}...", addr, port);
    teapot.listen( |request| {
        engine.borrow_mut().process(request)
    });
}
