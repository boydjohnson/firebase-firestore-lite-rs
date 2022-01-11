use std::fs::copy;

use glob::glob;
use npm_rs::Npm;

fn main() {
    Npm::default().install(None).run("build").exec().unwrap();

    let js_file = glob("dist/index*.js").unwrap().next().unwrap().unwrap();

    copy(js_file, "dist/main.js").unwrap();
}
