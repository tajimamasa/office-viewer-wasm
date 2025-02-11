use clap::Parser;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};
use wasmtime::component::TypedFunc;
#[derive(Parser, Debug)]
struct Args {
    wasm_file: String
}

fn start(args: Args) -> anyhow::Result<()> {
    // Wasmの処理を行うEngineオブジェクトを標準の設定で作成します    
    let engine = Engine::default();

    // ファイルをロードしてComponentオブジェクトを作成します
    let component = Component::from_file(&engine, &args.wasm_file)?;
    let linker = Linker::new(&engine);    

    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &component)?;

    // インスタンスのエクスポートされた関数を取得します
    let hello_index = instance.get_export(&mut store,None,"hello-world").unwrap();
    let hello:TypedFunc<(),(String,)> = instance.get_typed_func(&mut store,hello_index).unwrap();

    // 関数を呼び出します
    let (result,) = hello.call(&mut store,())?;
    println!("{}",result);

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
