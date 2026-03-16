use sdl2::{
	pixels::Color,
	rect::Rect,
	render::{Canvas, TextureCreator},
	video::{Window, WindowContext},
};

use crate::clock::PlantState;

pub const WIN_W: u32 = 480;
pub const WIN_H: u32 = 640;

const SPRITE_SIZE: u32 = 192;

const BG:         Color = Color::RGB(18,  18,  24 );
const TEXT:       Color = Color::RGB(220, 220, 210);
const TEXT_DIM:   Color = Color::RGB(110, 110, 100);
const GREEN:      Color = Color::RGB(80,  180, 80 );
const YELLOW:     Color = Color::RGB(220, 180, 50 );
const RED:        Color = Color::RGB(210, 60,  60 );
const BALLOON_BG: Color = Color::RGB(30,  30,  40 );
const BALLOON_BD: Color = Color::RGB(80,  80,  100);

const ICON_CLOCK:  &str = "\u{f017}";
const ICON_DROP:   &str = "\u{f043}";
const ICON_MOON:   &str = "\u{f186}";
const ICON_SUN:    &str = "\u{f185}";
const ICON_SKULL:  &str = "\u{f54c}";
const ICON_LEAF:   &str = "\u{f06c}";
const ICON_TROPHY: &str = "\u{f091}";

fn rect(x: i32, y: i32, w: u32, h: u32) -> Rect { Rect::new(x, y, w, h) }

fn draw_text_raw(
	canvas:  &mut Canvas<Window>,
	creator: &TextureCreator<WindowContext>,
	font:    &sdl2::ttf::Font,
	text:    &str,
	x:       i32,
	y:       i32,
	color:   Color,
) -> i32 {
	if text.is_empty() { return y; }
	let surface = font.render(text).blended(color).unwrap();
	let tex     = creator.create_texture_from_surface(&surface).unwrap();
	let q       = tex.query();
	canvas.copy(&tex, None, Some(rect(x, y, q.width, q.height))).unwrap();
	y + q.height as i32
}

fn draw_bar(
	canvas: &mut Canvas<Window>,
	x: i32, y: i32,
	w: u32, h: u32,
	progress: f64,
	color: Color,
) {
	canvas.set_draw_color(Color::RGB(40, 40, 50));
	canvas.fill_rect(rect(x, y, w, h)).unwrap();
	let filled = ((w as f64) * progress.clamp(0.0, 1.0)) as u32;
	if filled > 0 {
		canvas.set_draw_color(color);
		canvas.fill_rect(rect(x, y, filled, h)).unwrap();
	}
	canvas.set_draw_color(Color::RGB(80, 80, 100));
	canvas.draw_rect(rect(x, y, w, h)).unwrap();
}

fn draw_balloon(
	canvas:  &mut Canvas<Window>,
	creator: &TextureCreator<WindowContext>,
	font:    &sdl2::ttf::Font,
	lines:   &[String],
	x: i32, y: i32,
	w: u32,
) {
	if lines.is_empty() { return; }
	let pad    = 10i32;
	let lh     = 20i32;
	let height = (lines.len() as i32) * lh + pad * 2;

	canvas.set_draw_color(BALLOON_BG);
	canvas.fill_rect(rect(x, y, w, height as u32)).unwrap();
	canvas.set_draw_color(BALLOON_BD);
	canvas.draw_rect(rect(x, y, w, height as u32)).unwrap();

	for (i, line) in lines.iter().enumerate() {
		draw_text_raw(canvas, creator, font, line,
			x + pad,
			y + pad + (i as i32) * lh,
			TEXT,
		);
	}
}

// ── Tela de menu ──────────────────────────────────────────────

pub fn screen_menu(
	canvas:  &mut Canvas<Window>,
	creator: &TextureCreator<WindowContext>,
	font:    &sdl2::ttf::Font,
	t:       f64,
) {
	use sdl2::image::LoadTexture;

	canvas.set_draw_color(BG);
	canvas.clear();

	let cx = (WIN_W / 2) as i32;

	// ── Título: bob suave no Y + rotação leve ──
	let title_w   = 280u32;
	let title_h   = 100u32;
	let title_x   = cx - (title_w / 2) as i32;
	let title_bob = (t * 1.2).sin() * 8.0;
	let title_y   = 170 + title_bob as i32;
	let title_rot = (t * 0.8).sin() * 3.0;

	if let Ok(tex) = creator.load_texture("assets/sprites/title.png") {
		canvas.copy_ex(
			&tex,
			None,
			Some(rect(title_x, title_y, title_w, title_h)),
			title_rot,
			None,
			false, false,
		).unwrap();
	}

	// ── Botão PLAY: rotação ±45° em onda ──
	// seno rápido no ângulo → efeito ondinha/chacoalhar
	let btn_w   = 60u32;
	let btn_h   = 30u32;
	let btn_x   = cx - (btn_w / 2) as i32;
	let btn_y   = 370i32;
	let btn_rot = (t * 2.5).sin() * 45.0;

	// alterna pressed/normal junto com a onda
	let btn_path = if (t * 2.5).sin() > 0.0 {
		"assets/sprites/playbutton_pressed.png"
	} else {
		"assets/sprites/playbutton.png"
	};

	if let Ok(tex) = creator.load_texture(btn_path) {
		canvas.copy_ex(
			&tex,
			None,
			Some(rect(btn_x, btn_y, btn_w, btn_h)),
			btn_rot,
			None,
			false, false,
		).unwrap();
	}

	// ── Hint ──
	draw_text_raw(canvas, creator, font,
		"Enter ou Espaco para jogar",
		cx - 115, WIN_H as i32 - 50, TEXT_DIM);

	canvas.present();
}

// ── Tela de morte ─────────────────────────────────────────────

pub fn screen_death(
	canvas:  &mut Canvas<Window>,
	creator: &TextureCreator<WindowContext>,
	font:    &sdl2::ttf::Font,
	font_lg: &sdl2::ttf::Font,
	state:   &PlantState,
	sermon:  &[String],
	sprite:  &sdl2::render::Texture,
) {
	canvas.set_draw_color(Color::RGB(8, 4, 4));
	canvas.clear();

	let cx = (WIN_W / 2) as i32;

	draw_text_raw(canvas, creator, font_lg,
		&format!("{} PLANTA MORTA {}", ICON_SKULL, ICON_SKULL),
		cx - 110, 30, Color::RGB(160, 30, 30));

	let sx = cx - (SPRITE_SIZE / 2) as i32;
	canvas.copy(sprite, None, Some(rect(sx, 90, SPRITE_SIZE, SPRITE_SIZE))).unwrap();

	let sa = format_time(state.secs_alive());
	draw_text_raw(canvas, creator, font,
		&format!("{} sobreviveu {} dias  •  {}", state.name, state.days_lived, sa),
		30, 300, TEXT_DIM);

	let mut cy = 340;
	for line in sermon {
		cy = draw_text_raw(canvas, creator, font, line, 30, cy, Color::RGB(200, 80, 80));
		cy += 5;
	}

	draw_text_raw(canvas, creator, font,
		"[R] Nova planta    [Esc] Menu    [Q] Sair",
		30, WIN_H as i32 - 46, TEXT_DIM);

	canvas.present();
}

// ── Tela principal ────────────────────────────────────────────

pub fn screen_main(
	canvas:  &mut Canvas<Window>,
	creator: &TextureCreator<WindowContext>,
	font:    &sdl2::ttf::Font,
	font_lg: &sdl2::ttf::Font,
	state:   &PlantState,
	speech:  &[String],
	sprite:  &sdl2::render::Texture,
) {
	canvas.set_draw_color(BG);
	canvas.clear();

	let cx   = (WIN_W / 2) as i32;
	let days = state.days_no_water;

	draw_text_raw(canvas, creator, font_lg,
		&format!("{}  {}  {}", ICON_LEAF, state.name, ICON_LEAF),
		cx - 80, 16, TEXT);

	draw_text_raw(canvas, creator, font,
		&format!("{}  {}", ICON_CLOCK, format_time(state.secs_alive())),
		24, 56, TEXT_DIM);

	draw_text_raw(canvas, creator, font,
		&format!("Dia {}   {}  Recorde: {} dias", state.days_lived, ICON_TROPHY, state.record_days),
		24, 74, TEXT_DIM);

	let bar_color = if state.sleeping {
		Color::RGB(80, 100, 180)
	} else {
		Color::RGB(80, 180, 80)
	};
	draw_bar(canvas, 24, 98, WIN_W - 48, 10, state.day_progress(), bar_color);
	let mode = if state.sleeping {
		format!("{}  dormindo  (4x)", ICON_MOON)
	} else {
		format!("{}  acordada", ICON_SUN)
	};
	draw_text_raw(canvas, creator, font, &mode, 24, 114, TEXT_DIM);

	let sprite_y = 144i32;
	let sx = cx - (SPRITE_SIZE / 2) as i32;
	canvas.copy(sprite, None, Some(rect(sx, sprite_y, SPRITE_SIZE, SPRITE_SIZE))).unwrap();

	let balloon_y = sprite_y + SPRITE_SIZE as i32 + 8;
	draw_balloon(canvas, creator, font, speech, 24, balloon_y, WIN_W - 48);

	let thirst_y     = balloon_y + 88;
	let thirst_prog  = 1.0 - (days as f64 / 7.0);
	let thirst_color = match days {
		0..=2 => GREEN,
		3..=4 => YELLOW,
		_     => RED,
	};
	draw_text_raw(canvas, creator, font,
		&format!("{}  sede  {}/7 dias sem água", ICON_DROP, days),
		24, thirst_y, TEXT_DIM);
	draw_bar(canvas, 24, thirst_y + 18, WIN_W - 48, 12, thirst_prog, thirst_color);

	canvas.set_draw_color(Color::RGB(40, 40, 55));
	canvas.fill_rect(rect(0, WIN_H as i32 - 56, WIN_W, 1)).unwrap();

	draw_text_raw(canvas, creator, font,
		"[R] Regar    [Z] Dormir / Acordar    [Esc] Menu",
		24, WIN_H as i32 - 42, TEXT_DIM);

	canvas.present();
}

// ── Util ──────────────────────────────────────────────────────

pub fn format_time(secs: f64) -> String {
	let s = secs as u64;
	format!("{:02}h {:02}m {:02}s", s / 3600, (s % 3600) / 60, s % 60)
}

pub fn sprite_path(days_no_water: u32, dead: bool) -> &'static str {
	if dead { return "assets/sprites/plant_dead.png"; }
	match days_no_water {
		0..=2 => "assets/sprites/plant_happy.png",
		3..=4 => "assets/sprites/plant_stressed.png",
		_     => "assets/sprites/plant_dying.png",
	}
}
