use npm_rs::Npm;

fn main() {
    println!("cargo:rerun-if-changed=index.js");
    println!("cargo:rerun-if-changed=package.json");

    Npm::default().install(None).run("build").exec().unwrap();
}
