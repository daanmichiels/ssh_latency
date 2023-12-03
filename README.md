# ssh_latency
Measure SSH latency. Run `ssh_latency --help` for usage and options.

## Typical usage
On Windows:
```
ssh_latency.exe ssh my_host
```

On Linux:
```
ssh_latency ssh my_host
```

On other operating systems: likely to work just fine, but you'll have to build it yourself.
If you have Rust set up, `cargo build` should do it.

## Tested on
I have tested this:
- on Windows, using WSL (`ssh_latency.exe wsl`); typical time is a few hundred microseconds with low variance after the first few iterations
- on Windows, connecting to a Raspberry Pi (`ssh_latency.exe ssh daan@pi`); both machines are connected wirelessly to the same router; typical time is 5-10 milliseconds, and variance is high
