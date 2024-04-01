use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let count = args.next().unwrap().parse::<usize>().unwrap();
    let template_path = PathBuf::from(args.next().unwrap());
    println!("reading template {template_path:?}");
    let template = std::fs::read_to_string(template_path).unwrap();
    println!("rendering {count} times the template");

    let render_options = mrml::prelude::render::RenderOptions::default();
    println!("starting...");
    let start = std::time::Instant::now();
    for _ in 0..count {
        let parsed = mrml::parse(&template).unwrap();
        parsed.render(&render_options).unwrap();
    }
    let end = std::time::Instant::now();
    println!("done!");
    let duration = end.duration_since(start).as_secs_f64() * 1000.0;
    println!("executed in {duration}ms");
}
