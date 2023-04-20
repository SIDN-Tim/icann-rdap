use std::any::TypeId;

use icann_rdap_common::response::entity::Entity;

use super::{to_header, MdParams, ToMd};

impl ToMd for Entity {
    fn to_md(&self, params: MdParams) -> String {
        let typeid = TypeId::of::<Entity>();
        let mut md = String::new();
        md.push_str(&self.common.to_md(params.from_parent(typeid)));
        md.push_str(&to_header("Entity", params.heading_level, params.options));
        md.push_str(&self.object_common.to_md(params.from_parent(typeid)));
        md.push('\n');
        md
    }
}
