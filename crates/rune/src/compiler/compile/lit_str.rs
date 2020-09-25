use crate::compiler::compile::prelude::*;

/// Compile a literal string `"Hello World"`.
impl Compile<(&ast::LitStr, Needs)> for Compiler<'_> {
    fn compile(&mut self, (lit_str, needs): (&ast::LitStr, Needs)) -> CompileResult<()> {
        let span = lit_str.span();
        log::trace!("LitStr => {:?}", self.source.source(span));

        // NB: Elide the entire literal if it's not needed.
        if !needs.value() {
            self.warnings.not_used(self.source_id, span, self.context());
            return Ok(());
        }

        let string = lit_str.resolve(&self.storage, &*self.source)?;
        let slot = self.unit.new_static_string(&*string)?;
        self.asm.push(Inst::String { slot }, span);
        Ok(())
    }
}