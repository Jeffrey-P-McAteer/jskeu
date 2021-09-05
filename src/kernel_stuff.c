
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

#include <linux/fb.h>
#include <sys/ioctl.h>


bool got_screeninfo = false;
struct fb_var_screeninfo g_screeninfo = {};


int32_t activate_fb(int32_t fb_num) {
  // Call kernel, ideally only once
  if (!got_screeninfo) {
    if(0 > ioctl(fb_num, FBIOGET_VSCREENINFO, &g_screeninfo)) {
      perror("ioctl FBIOGET");
      return -1;
    }
    g_screeninfo.activate = 0;
    g_screeninfo.activate |= FB_ACTIVATE_NOW | FB_ACTIVATE_FORCE;
    got_screeninfo = true;
  }

  // Then tell it to flush screen
  if(0 > ioctl(fb_num, FBIOPUT_VSCREENINFO, &g_screeninfo)) {
    perror("ioctl FBIOPUT");
    return -1;
  }

  return 0;
}





