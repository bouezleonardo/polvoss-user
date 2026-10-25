use super::graphics_2d::{line_2d, point_2d, translate_2d, label_2d, rotate_2d,
                        refresh, set_pixel_char};
use super::syscalls::{sleep, read, STDIN, uptime, getchar};
use libm::{sinf, cosf};

/// Game box
fn square(sz: f32, dp: [f32; 2], ang: f32) {
  let mut p0: [f32;2] = [-sz, sz];
  let mut p1: [f32;2] = [sz, sz];
  let mut p2: [f32;2] = [sz, -sz];
  let mut p3: [f32;2] = [-sz, -sz];
  
  // rotate
  if ang != 0.0 {
    rotate_2d(&mut p0, ang);
    rotate_2d(&mut p1, ang);
    rotate_2d(&mut p2, ang);
    rotate_2d(&mut p3, ang);
  }

  // Translate
  if dp[0] != 0.0 && dp[1] != 0.0 {
    translate_2d(&mut p0, dp);
    translate_2d(&mut p1, dp);
    translate_2d(&mut p2, dp);
    translate_2d(&mut p3, dp);
  }
  
  // Lines
  line_2d(p0, p1);
  line_2d(p1, p2);
  line_2d(p2, p3);
  line_2d(p0, p3);
}

fn spawn_apple() {
  
}

static mut SEED: u32 = 12345;
pub fn random(min: u32, max: u32) -> f32 {
    unsafe {
        let mut x = SEED;

        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;

        SEED = x;

        (min + x % (max - min + 1)) as f32
    }
}

pub fn main_loop(){
    // Area inside the box
    const HEIGHT: f32 = 22.0;
    const WIDTH: f32 = 22.0;
    const BOX_CENTER: [f32;2] = [0.0, 0.0];
    const BOX_SIZE: f32 = 11.0;
    const BOX_AREA: usize = HEIGHT as usize * WIDTH as usize;
    const MIN_W: f32 = BOX_CENTER[0] - WIDTH/2.0;
    const MAX_W: f32 = BOX_CENTER[0] + WIDTH/2.0;
    const MIN_H: f32 = BOX_CENTER[1] - HEIGHT/2.0;
    const MAX_H: f32 = BOX_CENTER[1] + HEIGHT/2.0;
    
    // Current snake head position
    let mut snake_head: [f32;2] = [BOX_CENTER[0]-5.0, BOX_CENTER[1]];
    // All the positions where the head have been to
    let mut snake: [[f32;2];BOX_AREA] = [[0.0;2];BOX_AREA];
    snake[0] = snake_head;
    // Snake size    
    let mut snake_size: usize = 1;
    // Snake direction (w, s, a or d)
    let mut direction: u8 = b'd';
    
    // Apple location
    let mut apple: [f32;2] = [BOX_CENTER[0] + (-2.0), BOX_CENTER[1]];
    
    let mut run: bool = true;
    while run {
        label_2d("SNAKE GAME!", [BOX_CENTER[0]-5.0, BOX_SIZE + 4.0]);
        label_2d("Press 'q' to quit", [BOX_CENTER[0]-8.0, -BOX_SIZE-4.0]);
        
        // Game box
        set_pixel_char(b'#');
        square(BOX_SIZE, BOX_CENTER, 0.0);
        
        // Draw snake
        set_pixel_char(b'$');
        for i in 1..snake_size {
          point_2d([snake[i][0], snake[i][1]]);
        }
        set_pixel_char(b'@');
        point_2d(snake_head);
        
        // Draw apple
        set_pixel_char(b'o');
        point_2d(apple);
        
        //crate::print!("\x1B[{};{}HScore: {}", 0, 0, snake_size);
        
        crate::print!("\x1B[{};{}H Apple({},{}), Snake({}, {})", 0, 0, apple[0], apple[1], snake_head[0], snake_head[1]);
        
        sleep(8);
        
        // Refresh screen
        refresh();
        
        // Read key
        let key: u8 = getchar() as u8;
        if key == b'w' && direction != b's' {
          direction = b'w';
        } else if key == b's' && direction != b'w' {
          direction = b's';
        } else if key == b'a' && direction != b'd' {
          direction = b'a';
        } else if key == b'd' && direction != b'a' {
          direction = b'd';
        }
        
        // Move snake
        match direction {
            // Move snake
            b'w' => snake_head[1] += 1.0,
            b's' => snake_head[1] -= 1.0,
            b'a' => snake_head[0] -= 1.0,
            b'd' => snake_head[0] += 1.0,
            _ => ()
        }
        
        // Check if the snake is eating
        if snake_head == apple {
          snake_size += 1;
          
          // Set seed
          unsafe {
            SEED = uptime();
          }
          
          // Avoid spawning apple in the snake
          for j in 0..10000 {
            // New location for the apple
            apple[0] = random(MIN_W as u32 + 1, MAX_W as u32 - 1);
            apple[1] = random(MIN_H as u32 + 1, MAX_H as u32 - 1);
            
            for i in 1..snake_size {
              if apple == snake[i] {
                break;
              }
            }
          }
        }
        
        // Update snake body
        let mut i: usize = snake_size-1;
        snake[0] = snake_head;
        while i >= 1 {
          snake[i] = snake[i-1];
          i -= 1;
        }
        
        // Check if snake is hitting the wall
        if snake_head[0] >= MAX_W || snake_head[0] <= MIN_W ||
           snake_head[1] >= MAX_H || snake_head[1] <= MIN_H {
           run = false;
           break;
        }
        
        // Check if snake touching
        /*for i in 1..snake_size {
          if snake_head == snake[i]{
            run = false;
            break;
          }
        }*/
    }
}
