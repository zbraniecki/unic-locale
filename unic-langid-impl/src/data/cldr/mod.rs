pub mod layout_table {
    use crate::data::generate::get_layout_entry;
    use crate::CharacterDirection;
    use std::path::Path;
    use tinystr::TinyStr8;

    pub fn is_rtl(subtag: u64) -> bool {
        let path = "./data/cldr-misc-modern";

        let langid = unsafe { TinyStr8::new_unchecked(subtag) };
        let path = Path::new(path)
            .join("main")
            .join(&langid.to_string())
            .join("layout.json");

        let entry = get_layout_entry(path).expect("Entry retrival failed.");
        entry.2 == CharacterDirection::RTL
    }
}
