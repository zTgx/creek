use crate::primitives::CResult;

/// Read config from json file

pub trait RunningConfigReader {
	fn read(json_path: &str) -> CResult<RunningConfig>;
}

pub struct RunningConfig {
	pub sidechain_url: String,
}

impl RunningConfigReader for RunningConfig {
	fn read(_json_path: &str) -> CResult<RunningConfig> {
		todo!()
	}
}
