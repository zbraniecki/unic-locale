use unic_langid_impl::data::generate::generate_layout;

fn main() {
    let path = "./data/cldr-misc-modern";
    let (version, value) = generate_layout(path);
    println!("{}", value);
}
