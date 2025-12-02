(module
  (func $const_fold (export "const_fold")  (result i32) ;; line 1
    (local $x i32)
    (local $y i32)
    (local $z i32)
    ;; line 2
    i32.const 11
    local.set $x
    ;; line 3
    i32.const 8
    local.set $y
    ;; line 4
    local.get $x
    local.get $y
    i32.add
    local.set $z
    ;; line 5
    local.get $z
    return
    i32.const 0
  )
  (func $_start (export "_start") (result i32)
    (local $_result i32)
    ;; line 8
    call $const_fold
    local.set $_result
    local.get $_result
  )
)
