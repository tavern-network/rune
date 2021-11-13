use rune::termcolor::{ColorChoice, StandardStream};
use rune::{ContextError, Diagnostics, EmitDiagnostics, FromValue, Module, Source, Sources, Vm};
use std::sync::Arc;

fn main() -> rune::Result<()> {
    let m = module()?;

    let mut context = rune_modules::default_context()?;
    context.install(&m)?;

    let runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    sources.insert(Source::new("test", r#"pub fn main(a) { add(a) }"#));

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&context, &mut sources)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit_diagnostics(&mut writer, &sources)?;
    }

    let unit = result?;

    let mut vm = Vm::new(runtime, Arc::new(unit));
    let output = vm.call(&["main"], (1u32,))?;
    let output = i64::from_value(output)?;

    println!("{}", output);
    Ok(())
}

fn module() -> Result<Module, ContextError> {
    let mut m = Module::new();
    m.function(&["add"], |a: i64| a + 1)?;
    Ok(m)
}