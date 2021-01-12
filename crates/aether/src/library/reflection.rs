use crate::darksiders1::gfc;

impl gfc::Object {
    pub fn get_property(&self, name: &gfc::HString) -> Option<gfc::AutoRef<gfc::Value>> {
        let property = self.class().get_property_by_name(name, None)?;
        Some(property.get_value(self))
    }
}
