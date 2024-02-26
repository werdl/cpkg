use toml::Table;

#[derive(Debug)]
pub struct Pkg {
    web_url: String,
    name: String,
    version: String,
    depends_on: Vec<Pkg>,
    build_commands: Vec<String>,
}

#[derive(Debug)]
pub struct Registry {
    pkgs: Vec<Pkg>,
}

impl Registry {
    pub fn from_toml(toml: Table) -> Option<Registry> {
        // structure
        /*
        [cpkg.<name>]
        url = "https://..."
        version = "0.1.0"
        depends_on = ["<name>"]
        build_commands = ["make"]
                 */

        let mut pkgs = Vec::new();

        for (name, pkg) in toml.iter() {
            let pkg = pkg.as_table()?;
            
            println!("{:?}", pkg);
            let web_url = pkg.get("url")?.as_str()?.to_string();
            let version = pkg.get("version")?.as_str()?.to_string();
            let depends_on = pkg.get("depends_on")?.as_array()?;
            let build_commands = pkg.get("build_commands")?.as_array()?;

            let mut pkg = Pkg {
                web_url,
                name: name.to_string(),
                version,
                depends_on: Vec::new(),
                build_commands: Vec::new(),
            };

            for dep in depends_on {
                pkg.depends_on.push(Pkg {
                    web_url: "".to_string(),
                    name: dep.as_str()?.to_string(),
                    version: "".to_string(),
                    depends_on: Vec::new(),
                    build_commands: Vec::new(),
                });
            }

            for cmd in build_commands {
                pkg.build_commands.push(cmd.as_str()?.to_string());
            }

            pkgs.push(pkg);
        }

        Some(Registry { pkgs })
    }
}
