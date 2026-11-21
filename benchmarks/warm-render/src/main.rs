use std::{env,fs,time::{Instant,Duration},hint::black_box};
fn main(){let args:Vec<String>=env::args().collect();let source=fs::read_to_string(&args[1]).unwrap();let opts=mrml::prelude::render::RenderOptions::default();
let render=||mrml::parse(&source).unwrap().element.render(&opts).unwrap();
fs::write(&args[2],render()).unwrap();if env::var("CHECK_ONLY").is_ok(){return;}
let warm=Instant::now();while warm.elapsed()<Duration::from_secs(env::var("WARM_SECONDS").unwrap_or("5".into()).parse().unwrap()){black_box(render());}
let mut samples=Vec::new();let mut bytes=0usize;let start=Instant::now();while start.elapsed()<Duration::from_secs(env::var("BENCH_SECONDS").unwrap_or("10".into()).parse().unwrap()){let t=Instant::now();let html=render();bytes+=html.len();black_box(&html);samples.push(t.elapsed().as_secs_f64()*1000.);}
let elapsed=start.elapsed().as_secs_f64();samples.sort_by(|a,b|a.partial_cmp(b).unwrap());
let status=fs::read_to_string("/proc/self/status").unwrap();let rss=status.lines().find(|l|l.starts_with("VmHWM:")).unwrap().split_whitespace().nth(1).unwrap();
println!("{{\"engine\":\"mrml\",\"template\":\"{}\",\"count\":{},\"seconds\":{},\"rps\":{},\"p50_ms\":{},\"p95_ms\":{},\"bytes\":{},\"maxrss_kb\":{}}}",args[1],samples.len(),elapsed,samples.len() as f64/elapsed,samples[samples.len()/2],samples[samples.len()*95/100],bytes,rss);
}
