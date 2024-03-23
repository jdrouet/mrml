# Benchmark

```bash
# Building docker image
docker build -f benchmarks/Dockerfile -t mrml-bench .
# Running benchmark
docker run --rm mrml-bench /air-astana.mjml
docker run --rm mrml-bench /amario.mjml
```
