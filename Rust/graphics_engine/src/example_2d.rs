use super::graphics_2d::{line_2d, translate_window, put_window,
                        point_2d, translate_2d, label_2d, rotate_2d,
                        refresh, set_pixel_char};
use super::syscalls::{sleep, read, STDIN};
use libm::{sinf, cosf};

pub fn square(dp: [f32; 2], ang: f32) {
  let mut p0: [f32;2] = [-5.0, 5.0];
  let mut p1: [f32;2] = [5.0, 5.0];
  let mut p2: [f32;2] = [5.0, -5.0];
  let mut p3: [f32;2] = [-5.0, -5.0];
  
  // rotate
  rotate_2d(&mut p0, ang);
  rotate_2d(&mut p1, ang);
  rotate_2d(&mut p2, ang);
  rotate_2d(&mut p3, ang);
  
  // Translate
  translate_2d(&mut p0, dp);
  translate_2d(&mut p1, dp);
  translate_2d(&mut p2, dp);
  translate_2d(&mut p3, dp);
  
  // Lines
  line_2d(p0, p1);
  line_2d(p1, p2);
  line_2d(p2, p3);
  line_2d(p0, p3);
}

pub fn triangle(dp: [f32; 2], ang: f32) {
  let mut p0: [f32;2] = [-8.0, 8.0];
  let mut p1: [f32;2] = [8.0, 8.0];
  let mut p2: [f32;2] = [8.0, -8.0];
  
  // rotate
  rotate_2d(&mut p0, ang);
  rotate_2d(&mut p1, ang);
  rotate_2d(&mut p2, ang);
  
  // Translate
  translate_2d(&mut p0, dp);
  translate_2d(&mut p1, dp);
  translate_2d(&mut p2, dp);
  
  // Lines
  line_2d(p0, p1);
  line_2d(p1, p2);
  line_2d(p2, p0);
}

fn circle(pos: [f32;2], radius: f32) {
    let mut ang1: f32 = 0.0174532925;
    const FULL_ANG: f32 = 3.14159265*2.0;
    
    let mut i: usize = 0;
    
    // Array of points
    let mut points: [[f32;2]; 360] = [[0.0;2]; 360];
    
    // Generate points
    while i < 360 && ang1 < FULL_ANG {
        points[i] = [radius*cosf(ang1), radius*sinf(ang1)];
        
        // 1 degree in radians
        ang1 += 0.0174532925;
        
        i += 1;
    }
    
    // Transform points
    for j in 0..360 {
        translate_2d(&mut points[j], pos);
    }
    
    // Draw points
    for j in 0..360 {
        point_2d(points[j]);
    }
}

pub fn main_loop(){
    let mut ang: f32 = 0.0;
    
    // Square positions
    let s0: [f32;2] = [40.0, 5.0];
    let s1: [f32;2] = [-20.0, -5.0];
    
    // Triangle positions
    let t0: [f32;2] = [20.0, 20.0];
    
    // Moving point
    let mut p: [f32;2] = [0.0, 0.0];
    let label: &str = "You";
    
    loop {
        label_2d("Press 'q' to quit", [-8.0, 15.0]);
    
        // Squares
        set_pixel_char(b'#');
        square(s0, ang);
        square(s1, 0.0);
        circle([0.0,0.0], 10.0);
        
        triangle(t0, ang);
        
        // Player Character
        set_pixel_char(b'$');
        circle(p, 2.0);
        label_2d(label, [p[0]-1.0, p[1]+4.0]);
        put_window([p[0], p[1]/2.0]);
        
        // Refresh screen
        refresh();
        
        ang+= 0.017;
        
        // Key pressed
        let mut key: [u8; 2] = [0;2];
        let bytes: i32 = read(STDIN, key.as_mut_ptr(), 1);
        
        if bytes == 1 {
          match key[0] {
              // Move character
              b'w' => p[1] += 1.0,
              b's' => p[1] -= 1.0,
              b'a' => p[0] -= 1.0,
              b'd' => p[0] += 1.0,
              
              // Move camera
              b'i' => translate_window([0.0,1.0]),
              b'k' => translate_window([0.0,-1.0]),
              b'j' => translate_window([-1.0,0.0]),
              b'l' => translate_window([1.0,0.0]),
              
              b'q' => break,
              _ => ()
          }
        }
        label_2d("Press 'q' to quit", [-8.0, 15.0]);
    }
}
