use anyhow::Result;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use wasmtime::{Caller, Config, Engine, Func, Linker, Module, Store};
use wat::parse_str as wat2wasm;

#[derive(Clone)]
struct HostState {
    id: &'static str,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    config.epoch_interruption(true);

    let engine = Engine::new(&config)?;

    let wat = std::fs::read_to_string("demo.wat")?;
    let wasm = wat2wasm(&wat)?;
    let module = Module::new(&engine, &wasm)?;

    let mut linker = Linker::new(&engine);

    linker.func_wrap(
        "host",
        "log0",
        |mut caller: Caller<'_, HostState>, val: i32| {
            println!("[WASM][{}] log0: {}", caller.data().id, val);
        },
    )?;

    linker.func_wrap(
        "host",
        "log1",
        |mut caller: Caller<'_, HostState>, val: i32| {
            println!("[WASM][{}] log1: {}", caller.data().id, val);
        },
    )?;

    let mut store0 = Store::new(&engine, HostState { id: "T0" });
    let mut store1 = Store::new(&engine, HostState { id: "T1" });

    store0.epoch_deadline_async_yield_and_update(1);
    store1.epoch_deadline_async_yield_and_update(1);

    store0.set_epoch_deadline(1);
    store1.set_epoch_deadline(1);

    let instance0 = linker.instantiate_async(&mut store0, &module).await?;
    let instance1 = linker.instantiate_async(&mut store1, &module).await?;

    let run0 = instance0.get_typed_func::<(), ()>(&mut store0, "run0")?;
    let run1 = instance1.get_typed_func::<(), ()>(&mut store1, "run1")?;

    let engine_for_timer = engine.clone();
    let timer_task = tokio::spawn(async move {
        loop {
            engine_for_timer.increment_epoch();
            sleep(Duration::from_millis(5)).await;
        }
    });

    let t0 = tokio::spawn(async move {
        let mut store = store0;
        let f = run0;
        let _ = f.call_async(&mut store, ()).await;
    });

    let t1 = tokio::spawn(async move {
        let mut store = store1;
        let f = run1;
        let _ = f.call_async(&mut store, ()).await;
    });

    sleep(Duration::from_millis(200)).await;

    timer_task.abort();
    t0.abort();
    t1.abort();

    Ok(())
}
