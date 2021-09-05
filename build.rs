
fn main() -> Result<(), Box<dyn std::error::Error>> {
  cc::Build::new()
      .file("src/kernel_stuff.c")
      .warnings(true)
      .extra_warnings(true)
      .compile("libkernel_stuff.a");


  Ok(())
}

