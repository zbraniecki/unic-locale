use unic_langid_impl::data::generate::generate_likely_subtags;

fn main() {
    let path = "./data/cldr-core";
    let (_version, value) =
        generate_likely_subtags(path).expect("Generating likely subtags failed.");
    println!("{}", value);
}
