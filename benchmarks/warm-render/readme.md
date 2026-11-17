# Warm parse-and-render benchmark

This benchmark compares published **MRML 6.0.1** with **MJML 5.4.1** using
long-lived library processes. It complements the CLI startup benchmark in the
parent directory; the two workloads should not be compared directly.

## Environment and workload

- Host: Apple M1, Docker Desktop 4.37.2; Linux ARM64 VM, kernel
  `6.10.14-linuxkit`, eight logical CPUs and approximately 3.83 GiB VM RAM.
- Rust 1.91 release build; MRML default features plus `css-inline`.
- Node.js 22.23.5; MJML validation skipped and minification disabled. The async
  render API is awaited. MRML uses default rendering options.
- Each container: one logical CPU pinned to Linux CPU 2, one-core CPU quota,
  networking disabled. Resource runs disable swap and limit tasks to 64.
- Templates: 404 / 1,613 / 6,449 source bytes. Medium and large repeat a two-column
  product block three and fifteen times. These are synthetic fixtures, not a
  representative production corpus. They contain no explicit inline CSS rules.
- Every iteration parses and renders. Source input is loaded once, and there is
  no parsed-tree or output cache. HTML is consumed and discarded; one sample is
  written outside the timed interval. Disk/network/SMTP and per-recipient
  substitution are excluded.

## Recorded results

`recorded/throughput.jsonl` contains three ten-second measurements per
engine/template after five-second warm-ups. Engine order alternates by repeat.
`recorded/resources.jsonl` contains six additional 30-second measurements with
resource monitoring, followed by four ten-second reduced-memory checks. The
resource comparisons below each represent **one** 30-second measurement, not an
average across repeats. Host contention and monitoring can affect results.

Throughput and p50/p95 measure the rendering interval using monotonic clocks.
CPU accounting covers the full process lifecycle, including startup, sample
output, warm-up and final sorting. Do not divide lifecycle CPU seconds by the
measured render count to infer exact per-email CPU cost.

| Engine | Template | Renders/s | Average CPU | Peak CPU (100 ms sample) | Peak process RSS (MiB) | Peak container memory (MiB) | Peak sampled process threads |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| MJML | small | 2,197 | 99.4% | 104.0% | 108.2 | 68.1 | 7 |
| MRML | small | 68,341 | 99.6% | 100.3% | 27.0 | 25.2 | 1 |
| MJML | medium | 468 | 99.6% | 103.9% | 109.0 | 67.1 | 7 |
| MRML | medium | 12,194 | 99.5% | 113.2% | 8.8 | 7.2 | 1 |
| MJML | large | 86 | 99.2% | 102.0% | 109.7 | 67.9 | 7 |
| MRML | large | 2,905 | 99.6% | 100.9% | 4.7 | 5.0 | 1 |

CPU accounting comes from cgroup v2 `cpu.stat`; 100% means one logical CPU.
Sample boundaries and kernel accounting can produce short readings above 100%;
these do not imply more than one core of sustained capacity. The monitor is a
separate lightweight process in the same cgroup, and its overhead is included.
Thread sampling reads the renderer process, not the monitor, every 100 ms and
can miss short-lived threads. Runtime helper threads do not imply parallel
rendering workers.

Process RSS comes from Node's `process.resourceUsage().maxRSS` or Linux
`/proc/self/status` (`VmHWM`). Container memory comes from cgroup v2 `memory.peak`.
These have different accounting scopes, including how shared pages are counted,
and must not be summed. Host/VM memory is outside the container measurements.
The harness retains latency samples and sorts them at completion, increasing
memory use, especially for the fastest MRML case. Production queue buffers,
recipient data, HTTP layers and logging are absent.

The reduced-memory checks completed successfully for both small and large
fixtures with **256 MiB for MJML** and **64 MiB for MRML**, swap disabled. These
are the smallest limits tested, not absolute minimums or long-term stability
claims. The raw data includes CPU throttling counters and exit codes.

## One million renders: arithmetic projections

Dividing one million by the 30-second resource-run rates gives the following
pure-rendering estimates on one comparable CPU. These are not complete
million-recipient jobs, server recommendations or email delivery measurements.
The small MRML run did itself complete over two million repeated renders;
outputs were discarded and the template was unchanged.

| Template | MJML estimated rendering time | MRML estimated rendering time |
| --- | ---: | ---: |
| Small | 7.6 minutes | 14.6 seconds |
| Medium | 35.6 minutes | 82.0 seconds |
| Large | 3.24 hours | 5.7 minutes |

Deadline sizing can use `ceil(1,000,000 / (deadline_seconds * renders_per_second
* utilization_allowance))` workers as an initial estimate. This assumes ideal
scaling with one dedicated logical CPU per worker, which was not measured.
Validate actual templates and worker counts on the deployment hardware. Total
volume alone does not define minimum hardware without a completion deadline.

If every large-fixture HTML output is retained, one million outputs occupy
approximately 86.9 GB with MJML or 60.8 GB with MRML before filesystem/MIME
overhead. Stream outputs with bounded queues; storage or network bandwidth can
become the bottleneck.

## Compatibility scope

`recorded/compatibility.json` records six valid fixtures and three tolerance
probes. All six valid fixtures rendered with preserved sentinel content;
extracted links and image src/alt pairs matched in the common-component fixture.
The component fixture includes hero, accordion, carousel, social links, navbar,
tables and raw HTML in addition to basic layout components.

Both engines applied the tested inline CSS with MRML's `css-inline` enabled.
HTML was not byte-identical; serialization differed, including retention of
`!important` after applying an important color rule. MJML accepted a closing
`</br>` and an unclosed `<b>` that MRML rejected. The diagnostic Rust wrapper
uses `unwrap`, so parser errors produce exit code 101 in those probes; this does
not establish a crash inside MRML. Both returned HTML for the unknown-element
probe, so successful rendering alone does not establish input validation.

No email-client visual comparison, exhaustive attribute coverage, custom
component migration, include-loader testing or global compatibility percentage
is established by these checks. Use the actual templates and supported email
clients as a migration gate.

## Reproduce

From the repository root:

```sh
docker build -f benchmarks/warm-render/Dockerfile \
  -t mjml-mrml-bench:warm-render benchmarks/warm-render
python3 benchmarks/warm-render/run.py
python3 benchmarks/warm-render/resource_runs.py
python3 benchmarks/warm-render/compat.py
```

The scripts use Python's standard library. CPU 2 must be available; adjust
`--cpuset-cpus=2` in the runners for another machine. Resource monitoring requires
Linux cgroup v2, including `memory.peak`. Generated outputs go into the ignored
`results/` directory; recorded evidence is kept separately. `BENCHMARK_IMAGE`
can select an alternative image tag. `WARM_SECONDS` and `BENCH_SECONDS` configure
the individual engine executables; runners select their own measured intervals.

Direct engine versions are pinned, but this standalone recipe does not pin base
image digests or transitive dependency resolutions. Node's major-version image
tag can resolve to a newer patch release. The original resource image ID was
`sha256:70fae3fab0d1030993694bfb5c27e117d2233f072a320b9566bbb7f3f77bb116`.
Rebuilding reruns the workload; it does not guarantee byte-identical binaries or
identical timings. The benchmark does not change the repository workspace,
application dependency configuration or existing lockfiles.
