Benchmarks (Taffy 0.9.2 from crates.io vs Axes)

Runned on a Ryzen 5 5625U

| Benchmark | Axes | Taffy | Difference |
| --- | --- | --- | --- |
| Tree: 1,000 Nodes | 15.889 µs | 89.114 µs | 139.472% |
| Tree: 10,000 Nodes | 1.1186 µs | 1.0698 ms | 4.45988% |
| Tree: 100,000 Nodes | 8.7379 ms | 33.083 ms | 116.426% |
| Compute: 1,000 Nodes | 17.423 µs | 39.317 µs | 77.1731% |
| Compute: 10,000 Nodes | 176.51 µs | 446.35 µs | 86.6455% |
| Compute: 100,000 Nodes | 1.7988 ms | 14.976 ms | 157.107% |

Axes is about 2x faster than Taffy
