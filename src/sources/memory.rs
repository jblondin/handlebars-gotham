use sources::{Source, SourceError};
use std::collections::BTreeMap;
use handlebars::Handlebars;

pub struct MemorySource(pub BTreeMap<String, String>);

impl Source for MemorySource {
    fn load(&self, reg: &mut Handlebars) -> Result<(), SourceError> {
        for (name, tpl) in self.0.iter() {
            reg.register_template_string(name, tpl.clone())?
        }
        Ok(())
    }
}
