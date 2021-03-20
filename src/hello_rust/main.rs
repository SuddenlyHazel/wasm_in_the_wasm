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
fn add_one(v : u64) -> Nat {
    unsafe {
        let wats = r#"
                (module
                 (type $i64_=>_i64 (func (param i64) (result i64)))
                 (memory $0 0)
                 (export "addOne" (func $module/addOne))
                 (export "memory" (memory $0))
                 (func $module/addOne (param $0 i64) (result i64)
                  local.get $0
                  i64.const 1
                  i64.add
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
            "addOne",
            &[RuntimeValue::from(v)],
            &mut NopExternals,
        ).expect("failed to execute export").unwrap();

        let out : u64 = FromRuntimeValue::from_runtime_value(option).unwrap();
        return Nat::from(out);
    }
}