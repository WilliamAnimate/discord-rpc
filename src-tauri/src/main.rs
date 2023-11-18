// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod console;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use lazy_static::lazy_static;

use std::sync::Mutex;

#[cfg(windows)]
use console::show_console;

// static_mut is unsafe rust 💀💀💀
lazy_static! {
    /*     oh my god rust is such a joke: the beginning of the saga     */

    static ref CLIENT_ID: Mutex<String> = Mutex::new(String::new());

    static ref DETAILS: Mutex<String> = Mutex::new(String::new());
    static ref STATE: Mutex<String> = Mutex::new(String::new());
    static ref TIMESTAMP_ENABLED: Mutex<bool> = Mutex::new(false);
    static ref BUTTON_ONE_TEXT: Mutex<String> = Mutex::new(String::new());
    static ref BUTTON_ONE_LINK: Mutex<String> = Mutex::new(String::new());
    static ref BUTTON_TWO_TEXT: Mutex<String> = Mutex::new(String::new());
    static ref BUTTON_TWO_LINK: Mutex<String> = Mutex::new(String::new());
    static ref LARGE_IMAGE_ASSET_NAME: Mutex<String> = Mutex::new(String::new());
    static ref LARGE_IMAGE_ASSET_TEXT: Mutex<String> = Mutex::new(String::new());
    static ref SMALL_IMAGE_ASSET_NAME: Mutex<String> = Mutex::new(String::new());
    static ref SMALL_IMAGE_ASSET_TEXT: Mutex<String> = Mutex::new(String::new());

    static ref RPC_UP: Mutex<bool> = Mutex::new(false);
}

// ohshit unsafe rust 💀💀
// gave up using lazy_static
static mut MULTITHREAD_ASSIST_REQUEST_STOP_RPC: bool = false;

#[tauri::command]
fn dbg_show_console() {
    #[cfg(windows)] {
        show_console();
    }
}

#[tauri::command]
fn rpc_stop_thread() { unsafe {
    // let mut y = MUTLITHREAD_ASSIST_REQUEST_STOP_RPC.lock().unwrap();
    // *y = true;
    MULTITHREAD_ASSIST_REQUEST_STOP_RPC = true;
}}

#[tauri::command]
fn set_client_id(x: &str) {
    let mut y = CLIENT_ID.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_details(x: &str) {
    // *DETAILS = x;
    let mut y = DETAILS.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_state(x: &str) {
    // *DETAILS = x;
    let mut y = STATE.lock().unwrap(); // TODO: make this a function or macro?
    *y = x.to_string();
}

#[tauri::command]
fn set_timestamp_enabled(x: bool) {
    let mut y = TIMESTAMP_ENABLED.lock().unwrap();
    *y = x;
}

#[tauri::command]
fn set_button_one_text(x: &str) {
    let mut y = BUTTON_ONE_TEXT.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_button_one_link(x: &str) {
    let mut y = BUTTON_ONE_LINK.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_button_two_text(x: &str) {
    let mut y = BUTTON_TWO_TEXT.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_button_two_link(x: &str) {
    let mut y = BUTTON_TWO_LINK.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_large_image_asset_name(x: &str) {
    let mut y = LARGE_IMAGE_ASSET_NAME.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_large_image_asset_text(x: &str) {
    let mut y = LARGE_IMAGE_ASSET_TEXT.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_small_image_asset_name(x: &str) {
    let mut y = SMALL_IMAGE_ASSET_NAME.lock().unwrap();
    *y = x.to_string();
}

#[tauri::command]
fn set_small_image_asset_text(x: &str) {
    let mut y = SMALL_IMAGE_ASSET_TEXT.lock().unwrap();
    *y = x.to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            dbg_show_console,
            rpc_stop_thread,
            set_client_id,
            set_details,
            set_state,
            set_timestamp_enabled,
            set_button_one_text,
            set_button_one_link,
            set_button_two_text,
            set_button_two_link,
            set_large_image_asset_name,
            set_large_image_asset_text,
            set_small_image_asset_name,
            set_small_image_asset_text,
            rich_presence_callback,
        ])

        .run(tauri::generate_context!())
        .expect("");
}

#[tauri::command]
fn rich_presence_callback() {
    // let _ = set_rich_presence(); // RUST REALLY SUCKS
    // println!("{:?}", set_rich_presence(client_id));
    println!("{:?}", set_rich_presence());
}

fn set_rich_presence() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::spawn(|| {
        println!("starting rich presence");
        /*     oh my god rust is such a joke: the saga continues     */
        let client_id = CLIENT_ID.lock().unwrap();
        let details = DETAILS.lock().unwrap();
        let state = STATE.lock().unwrap();
        let timestamp_enabled = TIMESTAMP_ENABLED.lock().unwrap();
        let button_one_text = BUTTON_ONE_TEXT.lock().unwrap();
        let button_one_link = BUTTON_ONE_LINK.lock().unwrap();
        let button_two_text = BUTTON_TWO_TEXT.lock().unwrap();
        let button_two_link = BUTTON_TWO_LINK.lock().unwrap();
        let large_image_asset_name = LARGE_IMAGE_ASSET_NAME.lock().unwrap();
        let large_image_asset_text = LARGE_IMAGE_ASSET_TEXT.lock().unwrap();
        let small_image_asset_name = SMALL_IMAGE_ASSET_NAME.lock().unwrap();
        let small_image_asset_text = SMALL_IMAGE_ASSET_TEXT.lock().unwrap();

        println!("Successfully obtained locks on all variables");
        println!("client id is {}", client_id);

        let mut client = DiscordIpcClient::new(&client_id).expect("wtf");

        let _ = client.connect();

        let mut activity_base = activity::Activity::new();
        let mut activity_assets = activity::Assets::new();

        // this is where we init the buttons veclist thing.
        // N.B. we clone here because I don't want to mess up everything ahead.
        let mut buttons_vec_list_thing = activity_base.clone().buttons(vec![activity::Button::new("", "")]);

        if *details != "" {
            println!("details enabled, value is '{}'", &details);
            activity_base = activity_base.details(&details);
        }

        if *state != "" {
            println!("state enabled, value is '{}'", &state);
            activity_base = activity_base.state(&state);
        }

        if *timestamp_enabled == true {
            println!("timestamp enabled");
            activity_base = activity_base.timestamps(activity::Timestamps::new().start(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64));
        }

        if *button_one_text != "" {
            if *button_one_link != "" {
                println!("button one enabled; text: '{}', link: '{}'", &button_one_text, &button_one_link);
                let mut tmp_is_using_btn_two = false;

                // TODO: fix indentation hell
                if *button_two_text != "" {
                    if *button_two_link != "" {
                        println!("button two enabled (alongside button one); text: '{}', link: '{}'", &button_one_text, &button_one_link);
                        tmp_is_using_btn_two = true;

                        buttons_vec_list_thing = activity_base.clone().buttons(vec![activity::Button::new(&button_one_text, &button_one_link), activity::Button::new(&button_two_text, &button_two_link)]);
                    }
                }

                // imma explain this code because i know for a fact that i'll be confused in the future:
                // the code above checks if its using btn2, and the vec! thing uhh gets overwritten, resulting in one button.
                // so if we're using button 2, then both the buttons are set in the above code, otherwise, it'll go down here.
                // this is guarded behind an if statement for 2 reasons:
                // - its here, so we don't set it above just for it to be overwritten instantly; performance sake.
                // so if button 2 is set, it won't run this code, thus, no overwriting the two buttons
                // if im somehow confused by this, this is my cue to talk to a therapist 💀
                if !tmp_is_using_btn_two {
                    // not using btn2, so we only set it once.
                    buttons_vec_list_thing = activity_base.clone().buttons(vec![activity::Button::new(&button_one_text, &button_one_link)]);
                }

                activity_base = buttons_vec_list_thing;
            }
        }

        if *large_image_asset_name != "" {
            println!("large image name: '{}'", &large_image_asset_name);
            activity_assets = activity_assets.large_image(&large_image_asset_name);

            if *large_image_asset_text != "" {
                println!("large image text: '{}'", &large_image_asset_text);
                activity_assets = activity_assets.large_text(&large_image_asset_text);
            }
        }

        if *small_image_asset_name != "" {
            println!("small image name: '{}'", &small_image_asset_name);
            activity_assets = activity_assets.small_image(&small_image_asset_name);

            if *small_image_asset_text != "" {
                println!("small image text: '{}'", &small_image_asset_text);
                activity_assets = activity_assets.small_text(&small_image_asset_text);
            }
        }

        println!("all RPC assets set --- setting RPC");
        // you cant use ? for some reason, am i too stupid to figure this out?
        let _ = client.set_activity(activity_base.assets(activity_assets));

        loop {
            unsafe {
                // this is multithreaded, might cause memory leaks 💀
                if MULTITHREAD_ASSIST_REQUEST_STOP_RPC {
                    MULTITHREAD_ASSIST_REQUEST_STOP_RPC = false;
                    println!("RPC exiting!");
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    Ok(())
}
