(module
  (func $field_get_i32_f
    (import "field" "get_i32")
    (param (;field_index;) i32) (param (;destination;) i32))

  (memory $mem (export "memory") 0)

  (func $init (export "init") 
    (memory.grow (i32.const 10))
    drop
    )

  (func $sum_f (export "sum") (param $x i32) (param $y i32) (result i32)
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (call $field_get_i32_f (i32.const 0) (i32.const 0))
    (i32.add (i32.load (i32.const 0)) (local.get $y))
  )
)
