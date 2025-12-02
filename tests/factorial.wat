(module
  (func $fact (export "fact") (param $n i32) (result i32) ;; line 1
    (local $result i32)
    ;; line 2
    i32.const 1
    local.set $result
    ;; line 3
    block $break_0
    loop $continue_0
    local.get $n
    i32.const 0
    i32.gt_s
    i32.eqz
    br_if $break_0
    ;; line 3
    ;; line 4
    local.get $result
    local.get $n
    i32.mul
    local.set $result
    ;; line 5
    local.get $n
    i32.const 1
    i32.sub
    local.set $n
    br $continue_0
    end
    end
    ;; line 7
    local.get $result
    return
    i32.const 0
  )
  (func $_start (export "_start") (result i32)
    (local $_result i32)
    ;; line 10
    i32.const 5
    call $fact
    local.set $_result
    local.get $_result
  )
)
