
use framebuffer::{Framebuffer, KdMode};
//use nix::ioctl_write_buf;

use std::os::unix::io::AsRawFd;

use evdev::RelativeAxisType;



extern "C" {
  fn activate_fb(fb_num: i32) -> i32;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let fb_path = std::env::var("FRAMEBUFFER").unwrap_or("/dev/fb0".to_string());
  eprintln!("fb_path={}", &fb_path);
  
  let mut state = State {
    fb: Framebuffer::new(&fb_path)?,
    tick: 0,
    cursors: vec![],
    exit: false,
  };
  let mut remaining_errors: usize = 6000;
  
  Framebuffer::set_kd_mode(KdMode::Graphics)?;

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

  Framebuffer::set_kd_mode(KdMode::Text)?;
  Ok(())
}


struct State {
  pub fb: Framebuffer,
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

  // For each known mouse read position



  Ok(())
}


fn frame(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
  let w = state.fb.var_screen_info.xres;
  let h = state.fb.var_screen_info.yres;
  let line_length = state.fb.fix_screen_info.line_length;
  let bytespp = state.fb.var_screen_info.bits_per_pixel / 8;

  let mut frame = vec![0u8; (line_length * h) as usize];



  Ok(())
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