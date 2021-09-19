

/*
 * Assuming r+g+b buffer in p, write the rgb values in at offsets 0,1,2
 */
#[macro_export]
macro_rules! write_pixel_rgb {
  ($p:expr, $r:expr, $g:expr, $b:expr) => {
    $p[0] = $r;
    $p[1] = $g;
    $p[2] = $b;
  }
}


/*
 * The "Print Unwrap" macro simply performs .unwrap(),
 * but prints errors to stdout and performs some control
 * flow (return, continue, break) instead of panicing.
 * This was copied directly from the v3 core library where plugins
 * made more re-use of the macro, here it is only used in permission checks
 * that are guaranteed to fail on win64 with a FAT32 filesystem.
 */
#[macro_export]
macro_rules! punwrap_r {
    ($e:expr, continue) => {
      match $e {
        Ok(val) => val,
        Err(e) => {
          eprintln!("{}:{} e={:#?}", file!(), line!(), e);
          continue;
        }
      }
    };
    ($e:expr, break) => {
      match $e {
        Ok(val) => val,
        Err(e) => {
          eprintln!("{}:{} e={:#?}", file!(), line!(), e);
          break;
        }
      }
    };
    ($e:expr, return) => {
      match $e {
        Ok(val) => val,
        Err(e) => {
          eprintln!("{}:{} e={:#?}", file!(), line!(), e);
          return;
        }
      }
    };
    ($e:expr, nop) => {
      match $e {
        Ok(val) => (),
        Err(e) => {
          eprintln!("{}:{} e={:#?}", file!(), line!(), e);
        }
      }
    };
    ($e:expr, tf) => { // yields true of expression .is_some()/.is_ok(), else false
      match $e {
        Ok(val) => true,
        Err(e) => {
          eprintln!("{}:{} e={:#?}", file!(), line!(), e);
          false
        }
      }
    };
}



