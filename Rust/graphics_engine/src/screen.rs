use crate::syscalls::{write, STDOUT};
use crate::print;

// Resolution
pub const COLS: usize = 80;
pub const ROWS: usize = 24;
pub const SCREEN_SIZE: usize = COLS*ROWS;

// Screen (in viewport coordinates)
static mut SCREEN: [u8; SCREEN_SIZE] = [b' '; SCREEN_SIZE];

// Copy of the previous screen to avoid drawing same pixels
static mut PREV_SCREEN: [u8; SCREEN_SIZE] = [b' '; SCREEN_SIZE];

// Char that will be printed as a pixel
static mut PIXEL_CHAR: u8 = b'#';

/// Draw a point directly on the SCREEN using viewport
/// coordinates
pub fn draw_point_viewport(p: [i32; 2]) {
    // cols + line*n_cols
    unsafe {
      SCREEN[p[0] as usize + p[1] as usize * COLS] = PIXEL_CHAR;
    }
}

/// Draw a line directly on the SCREEN using viewport
/// coordinates
/// Draw a line directly on the SCREEN using viewport
/// coordinates
pub fn draw_line_viewport(og: [i32; 2], dst: [i32; 2]) {
    // Line equation
    let mut x0: f32 = og[0] as f32;
    let mut y0: f32 = og[1] as f32;
    let mut x1: f32 = dst[0] as f32;
    let mut y1: f32 = dst[1] as f32;
    
    // Make sure ordering is preserved
    if og[0] > dst [0] {
        x0 = dst[0] as f32;
        y0 = dst[1] as f32;
        x1 = og[0] as f32;
        y1 = og[1] as f32;
    }
    
    // Auxiliary
    let mut x: f32 = x0;
    let mut y: f32 = y0;
    let mut m: f32;
    
    // Check if it is vertical x1 == x0
    if og[0] == dst[0] {
        // Make sure ordering is preserved
        if og[1] > dst [1] {
            y0 = dst[1] as f32;
            y1 = og[1] as f32;
            y = y0;
        }

        // Vertical Line    
        while y <= y1 {
            draw_point_viewport([x0 as i32,y as i32]);
            y += 1.0;
        }
    }else{
        while x <= x1 {         
            draw_point_viewport([x as i32,y as i32]);
            // Using the line equation y = (y1 - y0)/(x1 - x0)*(x-x0)+y0
            m = (y1 - y0)/(x1 - x0);
            y = m * (x - x0) + y0;
            
            // cos^2 = 1/(1+tan^2)
            x += 1.0/(1.0+m*m);
        }
    }
}

/// Clip the points in the window before converting to the viewport
pub fn 
clip(x0: &mut f32, y0: &mut f32, x1: &mut f32, y1: &mut f32, wc: [f32; 2]) 
-> bool{
    // Window limits
    let xmin: f32 = wc[0] - ((COLS as f32)/2.0 - 1.0);
    let xmax: f32 = wc[0] + (COLS as f32)/2.0 - 1.0;
    let ymin: f32 = wc[1] - ((ROWS as f32)/2.0 - 1.0);
    let ymax: f32 = wc[1] + (ROWS as f32)/2.0 - 1.0;
    
    // Auxiliary variables
    let mut x_aux: f32;
    let mut y_aux: f32;
    
    // Check if the line between the points is visible
    if (*x0 < xmin && *x1 < xmin) || (*x0 > xmax && *x1 > xmax) {
        return false;
    }
    
    if (*y0 < ymin && *y1 < ymin) || (*y0 > ymax && *y1 > ymax) {
        return false;
    }
    
    if *x0 < xmin || *x0 > xmax {
        if *x0 < xmin {
            x_aux = xmin;
        } else {
            x_aux = xmax;
        }

        y_aux = (*y1 - *y0) / (*x1 - *x0) * (x_aux - *x0) + *y0;

        if y_aux < ymin {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymin - *y0) + *x0;
            y_aux = ymin;
        } else if y_aux > ymax {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymax - *y0) + *x0;
            y_aux = ymax;
        }
        
        if x_aux < xmin || x_aux > xmax {
            return false;
        }

        *x0 = x_aux;
        *y0 = y_aux;     
    } else if *y0 < ymin || *y0 > ymax {
        if *y0 < ymin {
            y_aux = ymin;
        } else {
            y_aux = ymax;
        }

        x_aux = (*x1 - *x0) / (*y1 - *y0) * (y_aux - *y0) + *x0;

        if x_aux < xmin {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmin - *x0) + *y0;
            x_aux = xmin;
        } else if x_aux > xmax {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmax - *x0) + *y0;
            x_aux = xmax;
        }
        
        if y_aux < ymin || y_aux > ymax {
            return false;
        }        
        
        *x0 = x_aux;
        *y0 = y_aux;
    }

    if *x1 < xmin || *x1 > xmax {
        if *x1 < xmin {
            x_aux = xmin;
        } else {
            x_aux = xmax;
        }

        y_aux = (*y1 - *y0) / (*x1 - *x0) * (x_aux - *x0) + *y0;

        if y_aux < ymin {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymin - *y0) + *x0;
            y_aux = ymin;
        } else if y_aux > ymax {
            x_aux = (*x1 - *x0) / (*y1 - *y0) * (ymax - *y0) + *x0;
            y_aux = ymax;
        }
        
        if x_aux < xmin || x_aux > xmax {
            return false;
        }
        
        *x1 = x_aux;
        *y1 = y_aux;
    } else if *y1 < ymin || *y1 > ymax {
        if *y1 < ymin {
            y_aux = ymin;
        } else {
            y_aux = ymax;
        }

        x_aux = (*x1 - *x0) / (*y1 - *y0) * (y_aux - *y0) + *x0;

        if x_aux < xmin {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmin - *x0) + *y0;
            x_aux = xmin;
        } else if x_aux > xmax {
            y_aux = (*y1 - *y0) / (*x1 - *x0) * (xmax - *x0) + *y0;
            x_aux = xmax;
        }
        
        if y_aux < ymin || y_aux > ymax {
            return false;
        }
        
        *x1 = x_aux;
        *y1 = y_aux;
    }

    return true;
}

/// Convert point from window to viewport coordinates
pub fn window_to_viewport(p: [f32; 2], wc: [f32; 2]) -> Option<[i32;2]> {
    // Window limits
    let xmin: f32 = wc[0] - ((COLS as f32)/2.0 - 1.0);
    let xmax: f32 = wc[0] + (COLS as f32)/2.0 - 1.0;
    let ymin: f32 = wc[1] - ((ROWS as f32)/2.0 - 1.0);
    let ymax: f32 = wc[1] + (ROWS as f32)/2.0 - 1.0;
    
    let mut x: f32 = p[0];
    let mut y: f32 = p[1]/2.0; // Divide to account for the distortion of the terminal
    
    if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
        // Convert window coordinates to viewport coordinates
        x = (x - xmin)*(COLS as f32 - 1.0)/(xmax-xmin);
        y = (y - ymin)*(ROWS as f32 - 1.0)/(ymax-ymin);
        
        // Draw on SCREEN
        return Some([x as i32, y as i32]);
    }
    None
}

/// Converts from the 2D Window coordinates to the viewport before
/// drawing a point
pub fn draw_point_window(p: [f32; 2], wc: [f32; 2]) {
    // Window limits
    let xmin: f32 = wc[0] - ((COLS as f32)/2.0 - 1.0);
    let xmax: f32 = wc[0] + (COLS as f32)/2.0 - 1.0;
    let ymin: f32 = wc[1] - ((ROWS as f32)/2.0 - 1.0);
    let ymax: f32 = wc[1] + (ROWS as f32)/2.0 - 1.0;
    
    let mut x: f32 = p[0];
    let mut y: f32 = p[1]/2.0; // Divide to account for the distortion of the terminal
    
    if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
        // Convert window coordinates to viewport coordinates
        x = (x - xmin)*(COLS as f32 - 1.0)/(xmax-xmin);
        y = (y - ymin)*(ROWS as f32 - 1.0)/(ymax-ymin);
        
        // Draw on SCREEN
        draw_point_viewport([x as i32, y as i32]);
    }
}

/// Converts from the 2D Window coordinates to the viewport before
/// drawing a line
pub fn draw_line_window(og: [f32; 2], dst: [f32; 2], wc: [f32; 2]){
    // Window limits
    let xmin: f32 = wc[0] - ((COLS as f32)/2.0 - 1.0);
    let xmax: f32 = wc[0] + (COLS as f32)/2.0 - 1.0;
    let ymin: f32 = wc[1] - ((ROWS as f32)/2.0 - 1.0);
    let ymax: f32 = wc[1] + (ROWS as f32)/2.0 - 1.0;
    
    let mut x0: f32 = og[0];
    // Divide to account for the distortion of the terminal
    let mut y0: f32 = og[1]/2.0;
    let mut x1: f32 = dst[0];
    // Divide to account for the distortion of the terminal
    let mut y1: f32 = dst[1]/2.0;
    
    // See if the line is visible after clipping
    let line_visible: bool;

    line_visible = clip(&mut x0, &mut y0, &mut x1, &mut y1, wc);
    
    if line_visible {
        // Convert window coordinates to viewport coordinates
        x0 = (x0 - xmin)*(COLS as f32 - 1.0)/(xmax-xmin);
        y0 = (y0 - ymin)*(ROWS as f32 - 1.0)/(ymax-ymin);
        
        x1 = (x1 - xmin)*(COLS as f32 - 1.0)/(xmax-xmin);
        y1 = (y1 - ymin)*(ROWS as f32 - 1.0)/(ymax-ymin);
        
        // Draw on SCREEN
        draw_line_viewport([x0 as i32, y0 as i32], [x1 as i32, y1 as i32]);
    }
}

pub fn clear_screen() {
    // Aquire lock
    unsafe {
      SCREEN = [b' '; SCREEN_SIZE];
    }
    // Go to position (0,0)
    print!("\x1B[H");
}

pub fn print_screen() {
    let mut offset: usize;
    
    for i in 0..ROWS {
        for j in 0..COLS {
            // Print pixel
            unsafe {
              offset = j + (ROWS-1-i)*COLS;
              if PREV_SCREEN[offset] != SCREEN[offset] {
                // Move cursor
                print!("\x1B[{};{}H",i,j);
                print!("{}", SCREEN[offset] as char);
                PREV_SCREEN[offset] = SCREEN[offset]; 
              }
            }
        }
    }

    /*let mut i: usize = ROWS-1;
    while i > 0 {
        // Get the pointer to the row
        let ptr: *const u8 = unsafe {
            core::ptr::addr_of!(SCREEN).cast::<u8>().add(i*COLS)
        };
        write(STDOUT, ptr, COLS); 
        write(STDOUT, "\n\r".as_ptr(), 2);
        i -= 1;
    }*/
}
/// Change the character being used as pixel
pub fn pixel_char(chr: u8){
  unsafe {    
    PIXEL_CHAR = chr;
  }
}
/// Get the character being used as pixel
pub fn current_pixel_char() -> u8 {
    unsafe {
      PIXEL_CHAR
    }
}
