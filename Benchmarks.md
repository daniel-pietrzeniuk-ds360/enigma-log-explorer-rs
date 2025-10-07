# Benchmarks

## .NET
```sh
Measure-Command { enigma-log-explorer -f .\intermediate_layer_logs.log 806 }
```
14369494 ticks (436ms)
12368615 ticks (236ms)
12929324 ticks (292ms)
12683392 ticks (268ms)
about: 260ms

## Rust
```sh
Measure-Command { enigma-log-explorer-rs session -fo .\intermediate_layer_logs.log 806 }
```
3490562 ticks (349ms)
3527198 ticks (352ms)
3131628 ticks (313ms)
about: 300ms

## Conclusions
C# version is faster but is less consistent,
also, when running `Measure-Command` it sometimes produces totalMilliseconds about a second longer than than Milliseconds
C# probably does better string handling than naive rs
Differences are rather small, soo I'm sticking with rs solution as it has better CLI.
