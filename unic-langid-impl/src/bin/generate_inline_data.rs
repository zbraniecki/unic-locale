use std::fs::File;
use std::io::Write;
use std::path::Path;
use unic_langid_impl::data::generate::{generate_layout, generate_likely_subtags};

fn main() {
    let misc_path = "./data/cldr-misc-modern";
    let core_path = "./data/cldr-core";

    let dest_path = "./src/data/inline";

    let (layout_version, layout_rs) = generate_layout(misc_path);
    let (main_version, likely_subtags_rs) =
        generate_likely_subtags(core_path).expect("Generating data failed");

    if layout_version != main_version {
        panic!("CLDR version mismatch!");
    }

    {
        let path = Path::new(dest_path).join("layout_table.rs");
        let mut f = File::create(path).expect("File doesn't exist");
        f.write_all(layout_rs.as_bytes()).expect("Writing failed");
    }

    {
        let path = Path::new(dest_path).join("likelysubtags").join("tables.rs");
        let mut f = File::create(path).expect("File doesn't exist");
        f.write_all(likely_subtags_rs.as_bytes())
            .expect("Writing failed");
    }

    {
        let version_rs = format!(r#"pub const CLDR_VERSION: &str = "{}";"#, main_version);
        let path = Path::new(dest_path).join("version.rs");
        let mut f = File::create(path).expect("File doesn't exist");
        f.write_all(version_rs.as_bytes()).expect("Writing failed");
    }
}
