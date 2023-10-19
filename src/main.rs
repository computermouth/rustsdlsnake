// all of this code in the run() function works in a no_std environment
// feel free to split this example into main.rs and lib.rs files

use fermium::prelude::*;
use rand::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Dn,
    Lt,
    Rt
}

struct ProgramState {
    quit: bool,
    ldir: Dir,
    ndir: Dir,
    snake: Vec<(usize, usize)>,
    apple: [[bool; 40]; 40],
    frame_i: isize,
    frame_n: isize,
    rng: ThreadRng,
}

fn new_ps() -> ProgramState {
    let mut ps = ProgramState {
        quit: false,
        ldir: Dir::Lt,
        ndir: Dir::Lt,
        snake: vec![(20, 20), (21, 20), (22, 20)],
        apple: [[false; 40]; 40],
        frame_i: 0,
        frame_n: 30,
        rng: rand::thread_rng()
    };
    ps.apple[10][10] = true;
    ps.apple[30][10] = true;
    ps.apple[30][30] = true;
    
    return ps;
}

fn init_sdl() -> *mut SDL_Renderer {

    unsafe { assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0) };
    let window = unsafe {
        SDL_CreateWindow(
            b"snake2\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            400,
            400,
            0,
        )
    };
    assert!(!window.is_null());

    let default_driver = -1;
    let renderer =
        unsafe { SDL_CreateRenderer(window, default_driver, SDL_RENDERER_PRESENTVSYNC.0) };
    assert!(!renderer.is_null());
    
    return renderer;
}

fn main() {
    let mut ps = new_ps();
    let renderer = init_sdl();
    
    while !ps.quit {
        update(&mut ps);
        render(renderer, &ps);
    }
    unsafe { SDL_Quit() };
}

fn update(ps: &mut ProgramState) {
	
    // handle quitting
    let mut event = SDL_Event::default();
    let pending_events = 0 < unsafe { SDL_PollEvent(&mut event) };
    if pending_events {
        let event_type = unsafe { event.type_ };
        match event_type {
            SDL_QUIT => ps.quit = true,
            SDL_KEYDOWN => match unsafe { event.key.keysym.sym } {
                SDLK_w | SDLK_UP => { if ps.ldir != Dir::Dn {ps.ndir = Dir::Up} },
                SDLK_s | SDLK_DOWN => { if ps.ldir != Dir::Up {ps.ndir = Dir::Dn} },
                SDLK_a | SDLK_LEFT => { if ps.ldir != Dir::Rt {ps.ndir = Dir::Lt} },
                SDLK_d | SDLK_RIGHT => { if ps.ldir != Dir::Lt {ps.ndir = Dir::Rt} }
            _ => {}
            },
            _ => {}
        }
    }
    
    // action frame
    if ps.frame_i == ps.frame_n {
        ps.frame_i = 0;
        
        // update position
        let mut next = ps.snake[0];
        match ps.ndir {
            Dir::Up => { next.1 = (next.1 + 40 - 1) % 40 },
            Dir::Dn => { next.1 = (next.1 +     41) % 40 },
            Dir::Lt => { next.0 = (next.0 + 40 - 1) % 40 },
            Dir::Rt => { next.0 = (next.0 +     41) % 40 },
        }
        ps.ldir = ps.ndir;
        
        // check collision with self
        if ps.snake.contains(&next) {
			*ps = new_ps();
			return;
        }
        
        ps.snake.insert(0, next);
        
        // if ate
		if ps.apple[next.0][next.1] == true {
			ps.apple[next.0][next.1] = false;
			let mut a: (usize, usize) = (ps.rng.gen_range(0..40), ps.rng.gen_range(0..40));
			while ps.apple[a.0][a.1] == true || ps.snake.contains(&(a.0, a.1)){
				a = (ps.rng.gen_range(0..40), ps.rng.gen_range(0..40));
			}
			if ps.snake.len() % 5 == 0 && ps.frame_n > 5 {
				ps.frame_n -= 5;
			}
			ps.apple[a.0][a.1] = true;
		} else {
			ps.snake.pop();
		}
    }
    
    ps.frame_i += 1;
}

fn set_draw_color(renderer: *mut SDL_Renderer, r: u8, g: u8, b: u8){
    unsafe {
        SDL_SetRenderDrawColor(
            renderer,
            r,
            g,
            b,
            255,
        );
    }
}

fn render(renderer: *mut SDL_Renderer, ps: &ProgramState) {
    // set background color
    set_draw_color(renderer, 0xFF, 0xF1, 0xE8);
    unsafe { SDL_RenderClear(renderer); }
    
    for s in ps.snake.iter(){
        let x: i32 = s.0 as i32 * 10 + 1;
        let y: i32 = s.1 as i32 * 10 + 1;
        let s = SDL_Rect {x: x, y: y, w: 8, h: 8};
        set_draw_color(renderer, 0xFF, 0x00, 0x4D);
        unsafe { SDL_RenderFillRect(renderer, &s); }
        set_draw_color(renderer, 0x7E, 0x25, 0x53);
        unsafe { SDL_RenderDrawRect(renderer, &s); }
    }
    
	for x in 0..40 {
		for y in 0..40 {
			if !ps.apple[x][y] {continue}
            let x: i32 = x as i32 * 10 + 1;
            let y: i32 = y as i32 * 10 + 1;
            let s = SDL_Rect {x: x, y: y, w: 8, h: 8};
            set_draw_color(renderer, 0x00, 0x87, 0x51);
            unsafe { SDL_RenderFillRect(renderer, &s); }
            set_draw_color(renderer, 0x00, 0xE4, 0x36);
            unsafe { SDL_RenderDrawRect(renderer, &s); }
		}
	}
    
    unsafe { SDL_RenderPresent(renderer); }
}

