use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_PATH: &str = "assets/config.toml";

const DEFAULT: &str = r#"# WhyYouForgetMe - Configuração
# Edite à vontade, sem precisar recompilar!

# Tempo real por dia virtual (em segundos)
# Padrão sugerido: 120.0 acordada, 30.0 dormindo
# Original tamagotchi: 600.0 acordada, 150.0 dormindo
secs_awake  = 120.0
secs_asleep = 30.0

# Dias sem água até a planta morrer
days_to_die = 7

# Nome da planta (deixe vazio "" para sortear um aleatório)
name = ""
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
	pub secs_awake:  f64,
	pub secs_asleep: f64,
	pub days_to_die: u32,
	pub name:        String,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			secs_awake:  120.0,
			secs_asleep: 30.0,
			days_to_die: 7,
			name:        String::new(),
		}
	}
}

pub fn load() -> Config {
	// cria o arquivo se não existir
	if !std::path::Path::new(CONFIG_PATH).exists() {
		let _ = fs::create_dir_all("assets");
		let _ = fs::write(CONFIG_PATH, DEFAULT);
		return Config::default();
	}

	let raw = fs::read_to_string(CONFIG_PATH).unwrap_or_default();
	toml::from_str(&raw).unwrap_or_else(|e| {
		eprintln!("Erro no config.toml: {e}. Usando valores padrão.");
		Config::default()
	})
}
