use deno_core::error::AnyError;
use deno_core::op;
use deno_core::Extension;

use std::rc::Rc;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;

    let runtime_extension = Extension::builder()
        .ops(vec![
            op_read_file::decl(),
            op_write_file::decl(),
            op_unlink::decl(),
        ])
        .build();

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runtime_extension],
        ..Default::default()
    });

    js_runtime
        .execute_script(
            "[playground-javascript-runtime:console.js]",
            include_str!("./runtime/console.js"),
        )
        .unwrap();

    js_runtime
        .execute_script(
            "[playground-javascript-runtime:fs.js]",
            include_str!("./runtime/fs.js"),
        )
        .unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_unlink(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js("./tests/task.js")) {
        eprintln!("error: {}", error);
    }
}
