use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use anyhow::Result;
use std::fs;
use toml;

pub trait TomlConfig {
	/// Reading TOML-config file and deserializing it into `Self` object
	fn parse<P>(pth: P) -> Result<Self>
	where
		P: AsRef<Path>,
		Self: Sized,
		for<'de> Self: Deserialize<'de>,
	{
		let content = fs::read_to_string(&pth)?;
		Ok(toml::from_str(&content)?)
	}

	/// Writing a serialized object `Self` to the TOML config file
	fn write<P>(&self, pth: P) -> Result<()>
	where
		Self: Serialize,
		P: AsRef<Path>,
	{
		let data = toml::to_string(&self)?;
		fs::write(&pth, data)?;

		Ok(())
	}
}
