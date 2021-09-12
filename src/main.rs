
use framebuffer::{Framebuffer, KdMode};
//use nix::ioctl_write_buf;

use std::os::unix::io::AsRawFd;
use std::process::Command;

use evdev::RelativeAxisType;



extern "C" {
  fn activate_fb(fb_num: i32) -> i32;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let fb_path = std::env::var("FRAMEBUFFER").unwrap_or("/dev/fb0".to_string());
  eprintln!("fb_path={}", &fb_path);
  
  let mut state = State {
    fb: Framebuffer::new(&fb_path)?,
    frame: None,
    tick: 0,
    cursors: vec![],
    exit: false,
  };
  let mut remaining_errors: usize = 900;
  
  let tty_to_return_to_ifn_in_tty = std::env::var("JSKEU_TTY_RETURN_NUM").unwrap_or("1".to_string()); // I always use tty1 as my x11 GUI
  let tty_to_go_to_ifn_in_tty = std::env::var("JSKEU_TTY_GOTO_NUM").unwrap_or("2".to_string()); // generally unused
  let we_were_in_tty = punwrap_r!(Framebuffer::set_kd_mode(KdMode::Graphics), tf);
  if (!we_were_in_tty) {
    eprintln!("Attempting to use chvt to go to tty {}", &tty_to_go_to_ifn_in_tty);
    punwrap_r!(Command::new("sudo")
      .args(&["chvt", &tty_to_go_to_ifn_in_tty])
      .status(), nop);
  }

  loop {

    // TODO let this be _much_ smarter
    std::thread::sleep(std::time::Duration::from_millis(1));

    remaining_errors -= 1;

    if let Err(e) = state_update(&mut state) {
      eprintln!("main state_update: {}", e);
      remaining_errors -= 1;
    }

    if let Err(e) = frame(&mut state) {
      eprintln!("main frame: {}", e);
      remaining_errors -= 1;
    }

    if state.exit {
      break;
    }

    if remaining_errors < 1 {
      eprintln!("Exiting because of too many errors!");
      break;
    }
  }

  if we_were_in_tty {
    // go back to cli mode
    punwrap_r!(Framebuffer::set_kd_mode(KdMode::Text), nop);
  }
  else {
    // switch to the x11 gui
    eprintln!("Attempting to use chvt to return to tty {}", &tty_to_return_to_ifn_in_tty);
    punwrap_r!(Command::new("sudo")
      .args(&["chvt", &tty_to_return_to_ifn_in_tty])
      .status(), nop);
  }
  
  Ok(())
}


struct State {
  pub fb: Framebuffer,
  pub frame: Option<Vec<u8>>,
  pub tick: u64,
  pub cursors: Vec<Cursor>,
  //pub keyboards: Vec<Keyboard>,
  pub exit: bool,
}

struct Cursor {
  // Used to read stuff
  pub device: evdev::Device,

  // Location of cursor on screen
  pub x: usize,
  pub y: usize,
  
  // whatever we read from upstream we multiply by this so users can have differenr cursor velocities,
  // starts as 1.0
  pub motion_v_multiplier: f32,


}


fn state_update(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
  state.tick = state.tick.wrapping_add(1);

  // ~once a second, scan for mice + add to state
  if (state.tick % 1000) == 0 {
    'devloop: for device in evdev::enumerate() {
      // skip if already known
      for known in &state.cursors {
        if known.device.physical_path().unwrap_or("") == device.physical_path().unwrap_or("") {
          continue 'devloop;
        }
      }
      
      // check if mouse + add cursors to state
      if device.supported_relative_axes().map_or(false, |axes| axes.contains(RelativeAxisType::REL_X)) {
        eprintln!("Detected mouse at {:?}", device.physical_path().unwrap_or(""));
        state.cursors.push(Cursor {
          device: device,
          x: 0,
          y: 0,
          motion_v_multiplier: 1.0,
        });
      }

      // TODO check if touchpad input (absolute axes) + add cursors to state


      // TODO check if keyboard + add to state

    }
  }

  // Every tick, for each known mouse read position
  for known in &mut state.cursors {
    known.x = 5;
  }


  Ok(())
}


fn frame(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
  let w = state.fb.var_screen_info.xres;
  let h = state.fb.var_screen_info.yres;
  let line_length = state.fb.fix_screen_info.line_length;
  let bytespp = state.fb.var_screen_info.bits_per_pixel / 8;

  if ! state.frame.is_some() {
    state.frame = Some(vec![0u8; (line_length * h) as usize]);
  }

  if let Some(ref mut frame) = &mut state.frame {
    
    // First draw BG (TODO not this, srsly bad performance)
    for (r, line) in frame.chunks_mut(line_length as usize).enumerate() {
      for (c, p) in line.chunks_mut(bytespp as usize).enumerate() {
        write_pixel_rgb!(p, 12, 12, 12);
      }
    }

    // 2nd draw mice
    for known in &state.cursors {
      // get i from x+y coords and draw
      


    }
    
    
    // Finally write + activate the frame
    state.fb.write_frame(&frame);
    
    unsafe {
      activate_fb(state.fb.device.as_raw_fd());
    }
  }

  Ok(())
}

/* Utilities */

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




/* old code:

let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();

    let w = framebuffer.var_screen_info.xres;
    let h = framebuffer.var_screen_info.yres;
    let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    let mut frame = vec![0u8; (line_length * h) as usize];

    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();

    for (r, line) in frame.chunks_mut(line_length as usize).enumerate() {
        for (c, p) in line.chunks_mut(bytespp as usize).enumerate() {
            let x0 = (c as f32 / w as f32) * 3.5 - 2.5;
            let y0 = (r as f32 / h as f32) * 2.0 - 1.0;

            let mut it = 0;
            let max_it = 200;

            let mut x = 0.0;
            let mut y = 0.0;

            while x * x + y * y < 4.0 && it < max_it {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                it += 1;
            }

            p[0] = (125.0 * (it as f32 / max_it as f32)) as u8;
            p[1] = (255.0 * (it as f32 / max_it as f32)) as u8;
            p[2] = (75.0 * (it as f32 / max_it as f32)) as u8;
        }
    }

    let _ = framebuffer.write_frame(&frame);

    unsafe {
      activate_fb(framebuffer.device.as_raw_fd());
    }

    std::io::stdin().read_line(&mut String::new()).unwrap();

    let _ = framebuffer.write_frame(&frame);

    unsafe {
      activate_fb(framebuffer.device.as_raw_fd());
    }

    std::io::stdin().read_line(&mut String::new()).unwrap();
    

    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();

*/