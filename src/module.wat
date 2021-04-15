(module
  ;;(func $component_create_f
  ;;  (import "component" "create")
  ;;  (param i32) (result i32))
  ;;(func $component_add_field_f
  ;;  (import "component" "field_add")
  ;;  (param i32) (param i32) (param i32))

  (func $field_get_i32_f
    (import "field" "get_i32")
    (param (;field_index;) i32) (param (;destination;) i32))
  ;;(func $field_get_i64_f
  ;;  (import "component" "field_get_i64")
  ;;  (param (;field_index;) i32) (param (;destination;) i32))
  ;;(func $field_get_f32_f
  ;;  (import "component" "field_get_f32")
  ;;  (param (;field_index;) i32) (param (;destination;) i32))
  ;;(func $field_get_f64_f 
  ;;  (import "component" "field_get_f64")
  ;;  (param (;field_index;) i32) (param (;destination;) i32))
  ;;(func $field_get_str_f
  ;;  (import "component" "field_get_str")
  ;;  (param (;field_index;) i32) (param (;destination;) i32) (param (;destination_length;) i32) (param (;string_start_index;) i32) (result (;length_from_start;) i32))

  ;;(func $field_set_i32_f
  ;;  (import "component" "field_set_i32")
  ;;  (param (;field_index;) i32) (param (;value;) i32))
  ;;(func $field_set_i64_f
  ;;  (import "component" "field_set_i64")
  ;;  (param (;field_index;) i32) (param (;value;) i64))
  ;;(func $field_set_f32_f
  ;;  (import "component" "field_set_f32")
  ;;  (param (;field_index;) i32) (param (;value;) f32))
  ;;(func $field_set_f64_f
  ;;  (import "component" "field_set_f64")
  ;;  (param (;field_index;) i32) (param (;value;) f64))
  ;;(func $field_set_str_f
  ;;  (import "component" "field_set_str")
  ;;  (param (;field_index;) i32) (param (;string_data;) i32) (param (;data_length;) i32))

  (memory $mem (export "memory") 0)


  (func $init (export "init") 
    (memory.grow (i32.const 10))
    drop
    )

  (func $sum_f (export "sum") (param $x i32) (param $y i32) (result i32)
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (call $field_get_i32_f (local.get $x) (i32.const 0))
    (i32.add (i32.load (i32.const 0)) (local.get $y))
    )
)
