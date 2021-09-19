
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


  Ok(())
}


