use clap::Parser;
use wasmtime::component::TypedFunc;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
#[derive(Parser, Debug)]
struct Args {
    wasm_file: String,
}

fn start(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &args.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &component)?;
    let index = instance.get_export(&mut store, None, "parse-file").unwrap();
    let func: TypedFunc<(Vec<u8>,), (String,)> =
        instance.get_typed_func(&mut store, index).unwrap();
    let input_data: Vec<u8> = vec![
        0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
    ];
    let (result,) = func.call(&mut store, (input_data,))?;
    println!("{}", result);

    Ok(())
}

fn main() {
    let args = Args::parse();
    println!("Wasm file: {}", args.wasm_file);
    if let Err(e) = start(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
