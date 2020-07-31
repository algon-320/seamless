use std::env;
use std::process::Command;

fn out_dir() -> String {
    env::var("OUT_DIR").unwrap()
}

fn build_ruby_binding() {
    const SRC: &str = "src/language/ruby.c";
    const LIBNAME: &str = "rubybind";

    fn ruby_config(key: &str) -> String {
        let out = Command::new("ruby")
            .arg("-e")
            .arg(&format!("print RbConfig::CONFIG['{}']", key))
            .output()
            .unwrap()
            .stdout;
        String::from_utf8_lossy(&out).to_string()
    }
    let lib_dir = format!(
        "-L {} -L {}",
        ruby_config("libdir"),
        ruby_config("archlibdir")
    );
    let lib_flag = format!("{} {}", ruby_config("LIBS"), ruby_config("LIBRUBYARG"));
    let include_dir1 = ruby_config("rubyhdrdir");
    let include_dir2 = ruby_config("rubyarchhdrdir");
    let ruby_so_name = ruby_config("RUBY_SO_NAME");

    cc::Build::new()
        .out_dir(&out_dir())
        .file(SRC)
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-g")
        .flag("-O0")
        .include(include_dir1)
        .include(&include_dir2)
        .flag(&lib_dir)
        .flag(&lib_flag)
        .compile(&format!("lib{}.a", LIBNAME));

    println!("cargo:rustc-link-lib=dylib={}", ruby_so_name);
    println!("cargo:rerun-if-changed={}", SRC);
}

fn main() {
    build_ruby_binding();
}
