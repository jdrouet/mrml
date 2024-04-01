# Benchmark

## Simple through the cli

The first benchmark attempt was made by comparing mjml and mrml-cli.

```bash
# Building docker image
docker build -f benchmarks/Dockerfile -t mrml-bench .
# Running benchmark
docker run --rm mrml-bench /air-astana.mjml
docker run --rm mrml-bench /amario.mjml
```

It gave some nice results (`mrml` being **more than 110 faster** than `mjml`) but by doing so, we also compare the efficient of nodejs of reading files from the filesystem, which is not what I'm looking for.

## Testing rendering in memory template

The second benchmark will take the previous side effects aside. We load the template in memory and just compare the time we need to execute 10000 rendering cycles for `mrml` and `mjml`.

```bash
# Building docker image
docker build -f benchmarks/stress.Dockerfile -t mrml-stress .
# Running benchmark
docker run --rm mrml-stress 10000 /air-astana.mjml
docker run --rm mrml-stress 10000 /amario.mjml
```

On a `x64_86` server with 64GB of RAM, Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz, 16 cores, I get the following results.

|                 | Difference | MRML           | MJML            |
| --------------- | ---------- | -------------- | --------------- |
| air-astana.mjml | x4.858     | 3011.477805ms  | 14630.734799ms  |
| amario.mjml     | x5.684     | 31947.511705ms | 181588.629815ms |

Giving a smaller gap.

If we limit the allocated resources to the created container by only allocating 64m of RAM and a single core.

```bash
docker run --rm --cpus=1 --memory=64m mrml-stress <count> <template>
```

|                 | Difference | MRML           | MJML            |
| --------------- | ---------- | -------------- | --------------- |
| air-astana.mjml | x8.152     | 3102.237748ms  | 25290.122476ms  |
| amario.mjml     | x11.020    | 32917.904679ms | 362756.333122ms |

We can see that the time is almost constant for `mrml` but increases for `mjml` even though we limited to 1 cores instead of 16.
