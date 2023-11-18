use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
extern "system" {
    fn AllocConsole() -> i32;
    fn ShowWindow(handle: *mut std::ffi::c_void, n_cmd_show: i32) -> i32;
    fn GetConsoleWindow() -> *mut std::ffi::c_void;
    fn SetConsoleTitleW(lp_console_title: *const u16) -> i32;
}

pub fn show_console() { unsafe {
	let console_title: Vec<u16> = OsStr::new("discord_rpc").encode_wide().chain(Some(0).into_iter()).collect();
	AllocConsole();
	SetConsoleTitleW(console_title.as_ptr());
	let console_handle = GetConsoleWindow();

	ShowWindow(console_handle, 5); // SW_SHOWNORMAL
}}
