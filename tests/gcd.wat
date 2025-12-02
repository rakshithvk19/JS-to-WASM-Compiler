(module
  (func $gcd (export "gcd") (param $a i32) (param $b i32) (result i32) ;; line 1
    (local $t i32)
    ;; line 2
    block $break_0
    loop $continue_0
    local.get $b
    i32.const 0
    i32.ne
    i32.eqz
    br_if $break_0
    ;; line 2
    ;; line 3
    local.get $b
    local.set $t
    ;; line 4
    local.get $a
    local.get $b
    i32.rem_s
    local.set $b
    ;; line 5
    local.get $t
    local.set $a
    br $continue_0
    end
    end
    ;; line 7
    local.get $a
    return
    i32.const 0
  )
  (func $_start (export "_start") (result i32)
    (local $_result i32)
    ;; line 10
    i32.const 48
    i32.const 18
    call $gcd
    local.set $_result
    local.get $_result
  )
)
