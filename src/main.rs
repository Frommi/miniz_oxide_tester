extern crate cargo;
extern crate crates_index;

use cargo::core::registry::PackageRegistry;
use cargo::core::PackageId;
use cargo::core::SourceId;
use cargo::util::Config;

fn main() {
    let index = crates_index::Index::new("crates.io".into());
    if !index.exists() {
        index.retrieve().expect("Could not fetch crates.io index");
    }

    let cfg = Config::default().unwrap();
    let src = SourceId::crates_io(&cfg).unwrap();
    for c in index.crates() {
        let mut reg = PackageRegistry::new(&cfg).unwrap();
        reg.add_sources(&[src.clone()]);
        let pkg = PackageId::new(
            c.name(),
            c.latest_version().version(),
            &src,
        ).unwrap();
        reg.get(&[pkg.clone()][..]).get(&pkg).unwrap();
        println!("got {}", c.name());
    }
}
