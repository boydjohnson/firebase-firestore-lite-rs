use npm_rs::Npm;

fn main() {
    Npm::default().install(None).run("build").exec().unwrap();
}
