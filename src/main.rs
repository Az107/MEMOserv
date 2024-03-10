use std::cell::RefCell;
mod engine;
mod memodb;
mod hteapot;
use engine::Engine;
use std::env;
use hteapot::HteaPot;

fn main() {
    
    print!("Starting server...");
    let args: Vec<String> = env::args().collect();
    let addr: String = args.get(1).unwrap_or(&String::from("0.0.0.0")).to_string();
    let port: String = args.get(2).unwrap_or(&String::from("8080")).to_string();
    let teapot = HteaPot::new(&addr, port.parse().unwrap());
    let engine = RefCell::new(Engine::new());
    engine.borrow_mut().init_mock_data();
    println!("started!");
    println!("Listening on {}:{}...", addr, port);
    teapot.listen( |request| {
        engine.borrow_mut().process(request)
    });
}
