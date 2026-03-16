use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, time::{Duration, SystemTime, UNIX_EPOCH}};

fn now() -> f64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or(Duration::ZERO)
		.as_secs_f64()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlantState {
	pub name:          String,
	pub days_lived:    u32,
	pub days_no_water: u32,
	pub secs_lived:    f64,
	pub sleeping:      bool,
	pub dead:          bool,
	pub record_days:   u32,
	pub tick_start:    f64,
	pub secs_awake:    f64,
	pub secs_asleep:   f64,
	pub days_to_die:   u32,
}

impl PlantState {
	pub fn new(name: String, secs_awake: f64, secs_asleep: f64, days_to_die: u32) -> Self {
		Self {
			name,
			days_lived:    0,
			days_no_water: 0,
			secs_lived:    0.0,
			sleeping:      false,
			dead:          false,
			record_days:   0,
			tick_start:    now(),
			secs_awake,
			secs_asleep,
			days_to_die,
		}
	}

	pub fn secs_per_day(&self) -> f64 {
		if self.sleeping { self.secs_asleep } else { self.secs_awake }
	}

	pub fn day_progress(&self) -> f64 {
		((now() - self.tick_start) / self.secs_per_day()).min(1.0)
	}

	pub fn secs_alive(&self) -> f64 {
		self.secs_lived + (now() - self.tick_start)
	}

	pub fn try_tick(&mut self) -> bool {
		if now() - self.tick_start < self.secs_per_day() {
			return false;
		}
		self.secs_lived    += now() - self.tick_start;
		self.tick_start     = now();
		self.days_lived    += 1;
		self.days_no_water += 1;
		if self.days_lived > self.record_days {
			self.record_days = self.days_lived;
		}
		if self.days_no_water >= self.days_to_die {
			self.dead = true;
		}
		true
	}

	pub fn water(&mut self) { self.days_no_water = 0; }

	pub fn toggle_sleep(&mut self) -> bool {
		self.sleeping = !self.sleeping;
		self.sleeping
	}

	pub fn reset(&mut self, new_name: String) {
		let (record, secs_awake, secs_asleep, days_to_die) = (
			self.record_days,
			self.secs_awake,
			self.secs_asleep,
			self.days_to_die,
		);
		*self = Self::new(new_name, secs_awake, secs_asleep, days_to_die);
		self.record_days = record;
	}
}

pub struct Clock {
	pub state: PlantState,
	path:      PathBuf,
}

impl Clock {
	pub fn load(path: PathBuf, cfg: &crate::config::Config) -> Self {
		let mut state = if path.exists() {
			let raw = fs::read_to_string(&path).unwrap_or_default();
			serde_json::from_str(&raw).unwrap_or_else(|_|
				PlantState::new(cfg.name.clone(), cfg.secs_awake, cfg.secs_asleep, cfg.days_to_die)
			)
		} else {
			PlantState::new(cfg.name.clone(), cfg.secs_awake, cfg.secs_asleep, cfg.days_to_die)
		};

		// Sempre atualiza config no boot (reflete mudanças no config.toml)
		state.secs_awake  = cfg.secs_awake;
		state.secs_asleep = cfg.secs_asleep;
		state.days_to_die = cfg.days_to_die;

		Self { state, path }
	}

	pub fn save(&self) {
		if let Some(p) = self.path.parent() { let _ = fs::create_dir_all(p); }
		let _ = fs::write(&self.path, serde_json::to_string_pretty(&self.state).unwrap_or_default());
	}
}
