
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let target = env::var("TARGET").unwrap();
  let target = target.as_str();

  if target.contains("linux") {
    cc::Build::new()
        .file("src/linux/framebuffer_apis.c")
        .warnings(true)
        .extra_warnings(true)
        .compile("libframebuffer_apis.a");

  }
  else if target.contains("windows") {
    // todo

  }
  else if target.contains("darwin") {
    // todo

  }

  generate_gl_bindings();

  Ok(())
}

fn generate_gl_bindings() {
  use gl_generator::{Api, Fallbacks, Profile, Registry};
  use std::path::PathBuf;
  use std::fs::File;

  let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
  let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Gles2, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(gl_generator::StructGenerator, &mut file)
        .unwrap();
}

