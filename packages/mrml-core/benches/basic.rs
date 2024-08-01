use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mrml::prelude::render::RenderOptions;

fn render(input: &str) {
    let opts = RenderOptions::default();
    let root = mrml::mjml::Mjml::parse(input).unwrap();
    root.element.render(&opts).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_str!("../resources/compare/success/mj-button.mjml");
    c.bench_function("render button", |b| b.iter(|| render(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
