use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use rand::prelude::*;

fn sdl_init() -> (sdl2::render::WindowCanvas, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 400, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let events = sdl_context.event_pump().unwrap();

    clear_cvs(&mut canvas);
    (canvas, events)
}

fn clear_cvs(cvs: &mut sdl2::render::WindowCanvas) {
    cvs.set_draw_color(Color::RGB(255, 255, 255));
    cvs.clear();
}

#[derive(PartialEq)]
enum Dir {
    Up,
    Dn,
    Lt,
    Rt
}

fn new_snake() -> Vec<(i8, i8)> {
    return vec![(20, 20), (21, 20), (22, 20)];
}

fn new_apples() -> [[bool; 40]; 40] {
    let mut apple = [[false; 40]; 40];
    apple[10][10] = true;
    apple[30][10] = true;
    apple[30][30] = true;
    return apple;
}

pub fn main() {
    let (mut cvs, mut events) = sdl_init();
    
    let mut rng = rand::thread_rng();
    
    let mut snake: Vec<(i8, i8)> = new_snake();
    let mut apple = new_apples();
    
    let mut dir = Dir::Lt;
    
    let mut frames = 0;
    let mut fr_cnt = 30;
    
    'running: loop {
        clear_cvs(&mut cvs);
            
        if frames == fr_cnt {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::W), .. } => if dir != Dir::Dn { dir = Dir::Up },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => if dir != Dir::Up { dir = Dir::Dn },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => if dir != Dir::Rt { dir = Dir::Lt },
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => if dir != Dir::Lt { dir = Dir::Rt },
                    _ => {}
                }
            }
            let mut next = snake[0];
            match dir {
                Dir::Up => { next.1 -= 1; if next.1 == -1 {next.1 = 39} },
                Dir::Dn => { next.1 += 1; if next.1 == 40 {next.1 =  0} },
                Dir::Lt => { next.0 -= 1; if next.0 == -1 {next.0 = 39} },
                Dir::Rt => { next.0 += 1; if next.0 == 40 {next.0 =  0} },
            }
            
            if snake.contains(&next) {
                snake = new_snake();
                apple = new_apples();
                continue 'running;
            }
            
            frames = 0;
            snake.insert(0, next);
        
            if apple[next.0 as usize][next.1 as usize] == true {
                apple[next.0 as usize][next.1 as usize] = false;
                let mut a: (usize, usize) = (rng.gen_range(0..40), rng.gen_range(0..40));
                while apple[a.0][a.1] == true || snake.contains(&(a.0 as i8, a.1 as i8)){
                    a = (rng.gen_range(0..40), rng.gen_range(0..40));
                }
                if snake.len() % 5 == 0 && fr_cnt > 5 {
                    fr_cnt -= 5;
                }
                apple[a.0][a.1] = true;
            } else {
                snake.pop();
            }
        }
        
        for s in snake.iter() {            
            let x: i32 = s.0 as i32 * 10 + 1;
            let y: i32 = s.1 as i32 * 10 + 1;
            // cvs.set_draw_color(Color::RGB(127, 0, 0));
            // cvs.fill_rect(Rect::new(x, y, 8, 8))
            //     .unwrap();
            cvs.set_draw_color(Color::RGB(255, 0, 0));
            cvs.draw_rect(Rect::new(x, y, 8, 8))
                .unwrap();
        }
        
		for x in 0..40 {
			for y in 0..40 {
				if !apple[x][y] {continue}
				let x: i32 = x as i32 * 10 + 1;
				let y: i32 = y as i32 * 10 + 1;
                // cvs.set_draw_color(Color::RGB(0, 127, 0));
                // cvs.fill_rect(Rect::new(x, y, 8, 8))
                //     .unwrap();
                cvs.set_draw_color(Color::RGB(0, 255, 0));
                cvs.draw_rect(Rect::new(x, y, 8, 8))
                    .unwrap();
			}
		}

        cvs.present();
        frames += 1;
    }
}
