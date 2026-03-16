mod brain;
mod clock;
mod config;
mod ui;

use clock::Clock;
use ui::{screen_death, screen_main, screen_menu, sprite_path, WIN_H, WIN_W};

use sdl2::{event::Event, keyboard::Keycode, image::{InitFlag, LoadTexture}};
use std::{
	path::PathBuf,
	sync::{Arc, Mutex},
	thread,
	time::{Duration, Instant},
};

#[derive(PartialEq)]
enum Tela { Menu, Jogo }

fn state_path() -> PathBuf {
	let base = std::env::var("HOME")
		.map(|h| format!("{h}/.local/share"))
		.unwrap_or_else(|_| ".".into());
	PathBuf::from(base).join("whyyouforgetme").join("state.json")
}

fn main() {
	let cfg = config::load();

	let sdl   = sdl2::init().expect("SDL2 init falhou");
	let video = sdl.video().expect("SDL2 video falhou");
	let _img  = sdl2::image::init(InitFlag::PNG).expect("SDL2_image init falhou");
	let ttf   = sdl2::ttf::init().expect("SDL2_ttf init falhou");

	let window = video
		.window("WhyYouForgetMe", WIN_W, WIN_H)
		.position_centered()
		.build()
		.expect("Janela falhou");

	let mut canvas = window
		.into_canvas()
		.accelerated()
		.present_vsync()
		.build()
		.expect("Canvas falhou");

	let creator = canvas.texture_creator();

	let font    = ttf.load_font("/usr/share/fonts/TTF/AgaveNerdFont-Regular.ttf", 15)
		.expect("Agave Regular não encontrada");
	let font_lg = ttf.load_font("/usr/share/fonts/TTF/AgaveNerdFont-Bold.ttf", 20)
		.expect("Agave Bold não encontrada");

	let clock = Arc::new(Mutex::new(Clock::load(state_path(), &cfg)));

	{
		let mut g = clock.lock().unwrap();
		if g.state.name.is_empty() {
			g.state.name = if cfg.name.is_empty() {
				brain::random_name()
			} else {
				cfg.name.clone()
			};
			g.save();
		}
	}

	let speech: Arc<Mutex<Vec<String>>> = {
		let g    = clock.lock().unwrap();
		let days = g.state.days_no_water;
		let name = g.state.name.clone();
		drop(g);
		Arc::new(Mutex::new(brain::gen_speech(&name, days)))
	};

	let mut event_pump = sdl.event_pump().unwrap();
	let mut sermon: Option<Vec<String>> = None;
	let mut tela   = Tela::Menu;
	let start      = Instant::now();

	'running: loop {
		let t = start.elapsed().as_secs_f64();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => break 'running,
				Event::KeyDown { keycode: Some(k), .. } => match tela {

					Tela::Menu => match k {
						Keycode::Return | Keycode::Space => tela = Tela::Jogo,
						Keycode::Q | Keycode::Escape     => break 'running,
						_ => {}
					},

					Tela::Jogo => {
						let dead = clock.lock().unwrap().state.dead;
						if dead {
							match k {
								Keycode::R => {
									let new_name = if cfg.name.is_empty() {
										brain::random_name()
									} else {
										cfg.name.clone()
									};
									{
										let mut g = clock.lock().unwrap();
										g.state.reset(new_name.clone());
										g.save();
									}
									sermon = None;
									let sp = Arc::clone(&speech);
									thread::spawn(move || {
										*sp.lock().unwrap() = brain::gen_speech(&new_name, 0);
									});
								}
								Keycode::Escape => tela = Tela::Menu,
								Keycode::Q      => break 'running,
								_ => {}
							}
						} else {
							match k {
								Keycode::R => {
									let name = {
										let mut g = clock.lock().unwrap();
										g.state.water();
										g.save();
										g.state.name.clone()
									};
									let sp = Arc::clone(&speech);
									thread::spawn(move || {
										*sp.lock().unwrap() = brain::gen_speech(&name, 0);
									});
								}
								Keycode::Z => {
									let (name, days) = {
										let mut g = clock.lock().unwrap();
										g.state.toggle_sleep();
										g.save();
										(g.state.name.clone(), g.state.days_no_water)
									};
									let sp = Arc::clone(&speech);
									thread::spawn(move || {
										*sp.lock().unwrap() = brain::gen_speech(&name, days);
									});
								}
								Keycode::Escape => tela = Tela::Menu,
								Keycode::Q      => break 'running,
								_ => {}
							}
						}
					}
				},
				_ => {}
			}
		}

		// tick só roda no jogo
		if tela == Tela::Jogo {
			let ticked = {
				let mut g = clock.lock().unwrap();
				let t = g.state.try_tick();
				if t { g.save(); }
				t
			};

			if ticked {
				let (name, days, dead, days_lived) = {
					let g = clock.lock().unwrap();
					(g.state.name.clone(), g.state.days_no_water, g.state.dead, g.state.days_lived)
				};
				if dead && sermon.is_none() {
					sermon = Some(vec!["...".into()]);
					let sp = Arc::clone(&speech);
					thread::spawn(move || {
						let s = brain::gen_death_speech(&name, days_lived);
						*sp.lock().unwrap() = s;
					});
				} else {
					let sp = Arc::clone(&speech);
					thread::spawn(move || {
						*sp.lock().unwrap() = brain::gen_speech(&name, days);
					});
				}
			}
		}

		// render
		match tela {
			Tela::Menu => {
				screen_menu(&mut canvas, &creator, &font, t);
			}
			Tela::Jogo => {
				let state = clock.lock().unwrap().state.clone();
				let sp    = speech.lock().unwrap().clone();

				let sprite_p = sprite_path(state.days_no_water, state.dead);
				let sprite   = creator.load_texture(sprite_p)
					.unwrap_or_else(|_| {
						let mut surf = sdl2::surface::Surface::new(
							128, 128, sdl2::pixels::PixelFormatEnum::RGBA8888
						).unwrap();
						let c = if state.dead                    { sdl2::pixels::Color::RGB(80,  60,  60)  }
						        else if state.days_no_water >= 5 { sdl2::pixels::Color::RGB(180, 60,  60)  }
						        else if state.days_no_water >= 3 { sdl2::pixels::Color::RGB(180, 160, 40)  }
						        else                             { sdl2::pixels::Color::RGB(60,  160, 60)  };
						surf.fill_rect(None, c).unwrap();
						creator.create_texture_from_surface(surf).unwrap()
					});

				if state.dead {
					let s = sermon.get_or_insert_with(|| sp.clone());
					screen_death(&mut canvas, &creator, &font, &font_lg, &state, s, &sprite);
				} else {
					screen_main(&mut canvas, &creator, &font, &font_lg, &state, &sp, &sprite);
				}
			}
		}

		thread::sleep(Duration::from_millis(16));
	}
}
