use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn render(input: &str) {
    mrml::to_html(input, mrml::Options::default()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_str!("../resources/template/amario.mjml");
    c.bench_function("amario", |b| b.iter(|| render(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
