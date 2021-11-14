#[cfg(debug_assertions)]
fn init_dotenv() {
	if let Err(e) = dotenv::dotenv() {
		eprintln!("dotenv failed to initialise: {}", e);
	} else {
		eprintln!("initialised dotenv successfully");
	}
}

#[cfg(not(debug_assertions))]
fn init_dotenv() {
	// dotenv is not used in production
}

pub struct Env {
	token: String,
	port: u16
}

impl Env {
	pub fn get_env() -> Env {
		init_dotenv();

		let token = env_var("TOKEN")
			.or("BOT_TOKEN")
			.get();

		let port = env_var("PORT")
			.get_or_default("7079".into())
			.parse::<u16>()
			.expect("failed to parse port");

		Env { token, port }
	}

	pub fn token(&self) -> &str {
		&self.token
	}

	pub fn port(&self) -> &u16 {
		&self.port
	}
}

// debug functions
#[cfg(debug_assertions)]
impl Env {
	#[inline]
	pub fn production(&self) -> bool { false }

	#[inline]
	pub fn development(&self) -> bool { true }
}

// production functions
#[cfg(not(debug_assertions))]
impl Env {
	#[inline]
	pub fn production(&self) -> bool { true }

	#[inline]
	pub fn development(&self) -> bool { false }
}

// struct that simplifies syntax of getting variables
pub struct EnvVar {
	var: Option<String>,
	tried_names: Vec<String>
}

pub fn env_var(name: &str) -> EnvVar {
	let var = std::env::var(name).ok();
	EnvVar { var, tried_names: vec![name.into()] }
}

impl EnvVar {
	pub fn or(mut self, name: &str) -> EnvVar {
		if self.var.is_none() {
			self.tried_names.push(name.into());
			self.var = std::env::var(name).ok();
		}
		self
	}

	pub fn get(self) -> String {
		// was this necessary? probably not.
		// i leave it here just cause its kinda funny i suppose lol
		// rust zero cost abstraction here? yes pls lol
		// i would be glad if rust's implementation is faster than whatever
		// woogly stuff this is

		// match self.var {
		// 	Some(var) => { var }
		// 	None => {
		//
		// 		let mut buf_len = self.tried_names.len() * 4;
		// 		self.tried_names.iter().for_each(|name| buf_len += name.len());
		//
		// 		let mut str_buf = String::with_capacity(buf_len);
		// 		self.tried_names.iter().enumerate().for_each(|(i, name)| {
		// 			if i > 0 { str_buf.push_str(", ") }
		// 			str_buf.push('"');
		// 			str_buf.push_str(name);
		// 			str_buf.push('"');
		// 		});
		//
		// 		panic!("could not find env variable. tried variables: {}", str_buf);
		// 	}
		// }

		self.var.expect(&format!(r#"could not find a suitable environment variable. tried variables: "{}""#, self.tried_names.join(r#"", ""#)))
	}

	pub fn get_or_default(self, default: String) -> String {
		self.var.unwrap_or(default.into())
	}
}
