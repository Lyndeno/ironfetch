# ironfetch

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)


A system fetching library and program written in rust.

The goal is to gather a comprehensive list of information about the system.

Minimal external crates are used. This is done for my own learning.

Sample output:
```console
      OS: NixOS 24.05 (uakari)
   Shell: bash
  Kernel: Linux 6.1.69 x86_64
   Model: Dell Inc. XPS 15 9560 05FFDN
Hostname: neo
  Uptime: 2 days, 23 hours, 31 minutes, 51 seconds
     CPU: Intel(R) Core(TM) i7-7700HQ CPU @ 2.80GHz (8/4) @ 1.400 GHz
  Memory: 9.30GiB / 62.66GiB DDR4
```