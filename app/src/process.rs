use serde::Deserialize;
use std::ffi::CStr;

// Terminal is not supported
#[derive(Deserialize, Clone)]
pub struct Process {
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub cwd: String,
}

impl Process {
    pub fn get_cwd(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.cwd.as_ptr() as *const i8) }
    }
    pub fn get_args(&self) -> Vec<&CStr> {
        self.args
            .clone()
            .unwrap_or(Vec::new())
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s.as_ptr() as *const i8) })
            .collect::<Vec<&CStr>>()
    }
    pub fn get_env(&self) -> Vec<&CStr> {
        self.env
            .clone()
            .unwrap_or(Vec::new())
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s.as_ptr() as *const i8) })
            .collect::<Vec<&CStr>>()
    }
}
