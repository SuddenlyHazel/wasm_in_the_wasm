use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use ic_cdk_macros::*;
use ic_cdk::export::candid::{Nat, TypeEnv};
use wasmi::{ImportsBuilder, Module, ModuleInstance, NopExternals, RuntimeValue, FromRuntimeValue, RuntimeArgs};

#[init]
fn init() {
    unsafe {
    }
}

#[query]
fn fib(v : u64) -> Nat {
    unsafe {
        let wats = r#"
            (module
             (type $i64_=>_i64 (func (param i64) (result i64)))
             (memory $0 0)
             (export "fib" (func $module/fib))
             (export "memory" (memory $0))
             (func $module/fib (param $0 i64) (result i64)
              (local $1 i32)
              (local $2 i32)
              (local $3 i32)
              i32.const 1
              local.set $1
              local.get $0
              i64.const 0
              i64.ne
              if
               loop $while-continue|0
                local.get $0
                i64.const 1
                i64.sub
                local.tee $0
                i64.const 0
                i64.ne
                if
                 local.get $1
                 local.get $2
                 i32.add
                 local.get $1
                 local.set $2
                 local.set $1
                 br $while-continue|0
                end
               end
               local.get $1
               i64.extend_i32_s
               return
              end
              i64.const 0
             )
            )
            "#;

        let binary = wat::parse_str(wats).unwrap();

        // Load wasm binary and prepare it for instantiation.
        let module = wasmi::Module::from_buffer(binary)
            .expect("failed to load wasm");

        // Instantiate a module with empty imports and
        // assert that there is no `start` function.
        let instance =
            ModuleInstance::new(
                &module,
                &ImportsBuilder::default()
            )
                .expect("failed to instantiate wasm module")
                .assert_no_start();

        let option = instance.invoke_export(
            "fib",
            &[RuntimeValue::from(v)],
            &mut NopExternals,
        ).expect("failed to execute export").unwrap();

        let out : u64 = FromRuntimeValue::from_runtime_value(option).unwrap();
        return Nat::from(out);
    }
}