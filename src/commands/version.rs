use clap::ArgMatches;

struct Version {
    version :i32,
}
impl Version {
    pub fn new() -> Version {
        Version{version:42}
    }

    pub fn version(&self) -> i32 {
        self.version
    }
}

pub fn run_version_cli(_: &ArgMatches) -> Result<(), String> {
    let ver = Version::new();
    println!("version: {}", ver.version());
    Ok(())
}
 