(module
  (func $dead_code (export "dead_code")  (result i32) ;; line 1
    (local $x i32)
    ;; line 2
    i32.const 5
    local.set $x
    ;; line 3
    local.get $x
    return
    i32.const 0
  )
  (func $_start (export "_start") (result i32)
    (local $_result i32)
    ;; line 9
    call $dead_code
    local.set $_result
    local.get $_result
  )
)
