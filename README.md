# oom_canary
Test what happens when cargo test blows up with OOM on github actions


## Question: What is the symptom when `cargo test` exceeds its memory limits on github runner?


```shell
cargo +nightly-2021-07-04 test -p oom --
```

## Answer:
```
     Running unittests (target/debug/deps/oom-dfbfb9f22ecf3c8b)

running 1 test
error: test failed, to rerun pass '--bin oom'

Caused by:
  process didn't exit successfully: `/home/runner/work/oom_canary/oom_canary/target/debug/deps/oom-dfbfb9f22ecf3c8b` (signal: 9, SIGKILL: kill)
Error: Process completed with exit code 101.
```


## Question: What is the symptom when `cargo miri` exceeds its memory limits on github runner?

```shell
cargo +nightly-2021-07-04 test -p oom --
```

## Answer:

```
     Running unittests (target/x86_64-unknown-linux-gnu/debug/deps/oom-eaaafdd2ac3e4a27)
running 1 test
memory allocation of 13107200000 bytes failed
error: test failed, to rerun pass '--bin oom'

Caused by:
  process didn't exit successfully: `/usr/share/rust/.rustup/toolchains/nightly-2021-07-04-x86_64-unknown-linux-gnu/bin/cargo-miri /home/runner/work/oom_canary/oom_canary/target/x86_64-unknown-linux-gnu/debug/deps/oom-eaaafdd2ac3e4a27` (exit status: 255)
test tests::oom ...
Error: Process completed with exit code 255.
```
