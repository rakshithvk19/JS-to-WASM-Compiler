(module
  (func $ack (export "ack") (param $m i32) (param $n i32) (result i32) ;; line 1
    ;; line 2
    local.get $m
    i32.const 0
    i32.eq
    if
    ;; line 2
    local.get $n
    i32.const 1
    i32.add
    return
    end
    ;; line 3
    local.get $n
    i32.const 0
    i32.eq
    if
    ;; line 3
    local.get $m
    i32.const 1
    i32.sub
    i32.const 1
    call $ack
    return
    end
    ;; line 4
    local.get $m
    i32.const 1
    i32.sub
    local.get $m
    local.get $n
    i32.const 1
    i32.sub
    call $ack
    call $ack
    return
    i32.const 0
  )
  (func $_start (export "_start") (result i32)
    (local $_result i32)
    ;; line 7
    i32.const 3
    i32.const 4
    call $ack
    local.set $_result
    local.get $_result
  )
)
