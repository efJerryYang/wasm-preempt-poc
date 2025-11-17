(module
  (import "host" "log0" (func $log0 (param i32)))
  (import "host" "log1" (func $log1 (param i32)))

  (func $run0 (export "run0")
    (local i32)        ;; local 0 = counter
    (loop
      local.get 0
      call $log0      ;; call host: log0(counter)
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      br 0            ;; infinite loop
    )
  )

  (func $run1 (export "run1")
    (local i32)
    (loop
      local.get 0
      call $log1
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      br 0
    )
  )
)

