## Project Structure

```
mkdir -p wasm-preempt
cd wasm-preempt/
git clone https://github.com/efJerryYang/wasm-preempt-poc.git
git clone https://github.com/efJerryYang/wasmtime.git && git checkout dev-poc-preempt
```

## Build the Demo

```
# from the wasm-preempt workspace root
cd wasmtime/ && cargo build -p wasmtime-cli --release
cd ../
cd wasm-preempt-poc/ && cargo build --release
```

## Run the Demo

```
cd wasm-preempt-poc/
timeout 0.2 cargo run --release > output.log   # adjust 0.2 as needed
```

You should see output similar to:

```
[POC][fiber] switching to fiber stack top = 0x7f5639ee8000
[POC][fiber] switching to fiber stack top = 0x7f5639ee8000
[POC][fiber] switching to fiber stack top = 0x7f5639ee8000
```

Because stdout is redirected to `output.log`, the `[POC]` logs appear on stderr, so they remain visible. This indicates that the runtime successfully triggered the context switch, which is the required starting point for implementing full preemption.

Only work on x86_64 Linux because I only edited the file for that platform, you can update other platform files as well in the wasmtime repository.

