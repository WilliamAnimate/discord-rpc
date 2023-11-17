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

    // static ref DETAILS: &'static str = "";

    // static ref DETAILS: Mutex<String> = Mutex::new("hey guys".to_string());
    // static ref STATE: Mutex<String> = Mutex::new("i refactored the entire application, if this is working then im gonna be so glad".to_string());
    // static ref TIMESTAMP_ENABLED: Mutex<bool> = Mutex::new(false);
    // static ref BUTTON_ONE_TEXT: Mutex<String> = Mutex::new("test".to_string());
    // static ref BUTTON_ONE_LINK: Mutex<String> = Mutex::new("https://github.com/williamanimate".to_string());
    // static ref BUTTON_TWO_TEXT: Mutex<String> = Mutex::new("test2".to_string());
    // static ref BUTTON_TWO_LINK: Mutex<String> = Mutex::new("https://therickroll.com".to_string());
    // static ref LARGE_IMAGE_ASSET_NAME: Mutex<String> = Mutex::new("niko".to_string());
    // static ref LARGE_IMAGE_ASSET_TEXT: Mutex<String> = Mutex::new("niko oneshot".to_string());
    // static ref SMALL_IMAGE_ASSET_NAME: Mutex<String> = Mutex::new("niko".to_string());
    // static ref SMALL_IMAGE_ASSET_TEXT: Mutex<String> = Mutex::new("niko oneshot".to_string());

    // static ref CLIENT: Mutex<DiscordIpcClient> = Mutex::new(DiscordIpcClient::new("1173464188787846165"));

    static ref CLIENT_ID: Mutex<String> = Mutex::new("".to_string());

    // N.B. this code doesn't work because the stupid process down there acquires the lock for this thing and doesn't let it edit
    // so we're using static mut, i don't know if it'll be memory safe, but it *should* be.
    // 💀💀💀
    // static ref MUTLITHREAD_ASSIST_REQUEST_STOP_RPC: Mutex<bool> = Mutex::new(false);

    static ref DETAILS: Mutex<String> = Mutex::new("".to_string());
    static ref STATE: Mutex<String> = Mutex::new("".to_string());
    static ref TIMESTAMP_ENABLED: Mutex<bool> = Mutex::new(false);
    static ref BUTTON_ONE_TEXT: Mutex<String> = Mutex::new("".to_string());
    static ref BUTTON_ONE_LINK: Mutex<String> = Mutex::new("".to_string());
    static ref BUTTON_TWO_TEXT: Mutex<String> = Mutex::new("".to_string());
    static ref BUTTON_TWO_LINK: Mutex<String> = Mutex::new("".to_string());
    static ref LARGE_IMAGE_ASSET_NAME: Mutex<String> = Mutex::new("".to_string());
    static ref LARGE_IMAGE_ASSET_TEXT: Mutex<String> = Mutex::new("".to_string());
    static ref SMALL_IMAGE_ASSET_NAME: Mutex<String> = Mutex::new("".to_string());
    static ref SMALL_IMAGE_ASSET_TEXT: Mutex<String> = Mutex::new("".to_string());

    // static ref TEST: Mutex<bool> = Mutex::new(false);
    static ref RPC_UP: Mutex<bool> = Mutex::new(false);
}

// ohshit unsafe rust 💀💀
// i just gave up trying to use lazy_static lmaooo
// static mut CLIENT: Result<DiscordIpcClient, Box<dyn std::error::Error>> = DiscordIpcClient::new("1173464118787846165");
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

// fn set_(x: &str) {
//     let mut d = A.lock().unwrap();
//     *d = x.to_string();
// }

fn _test_values_or_something_idfk() {
    set_details("detail");
    set_state("state");
    set_timestamp_enabled(true);
    set_button_one_text("btn1 txt");
    set_button_one_link("https://example.com/btn1_txt");
    set_button_two_text("btn2 txt");
    set_button_two_link("https://example.com/btn1_txt");
    set_large_image_asset_name("niko");
    set_large_image_asset_text("large image asset text");
    set_small_image_asset_name("niko");
    set_small_image_asset_text("small image asset text");
// // this comment serves no purpose other than to link everything so i can collapse it in one click.
//     let details = DETAILS.lock().unwrap();
//     let state = STATE.lock().unwrap();
//     let timestamp_enabled = TIMESTAMP_ENABLED.lock().unwrap();
//     let button_one_text = BUTTON_ONE_TEXT.lock().unwrap();
//     let button_one_link = BUTTON_ONE_LINK.lock().unwrap();
//     let button_two_text = BUTTON_TWO_TEXT.lock().unwrap();
//     let button_two_link = BUTTON_TWO_LINK.lock().unwrap();
//     let large_image_asset_name = LARGE_IMAGE_ASSET_NAME.lock().unwrap();
//     let large_image_asset_text = LARGE_IMAGE_ASSET_TEXT.lock().unwrap();
//     let small_image_asset_name = SMALL_IMAGE_ASSET_NAME.lock().unwrap();
//     let small_image_asset_text = SMALL_IMAGE_ASSET_TEXT.lock().unwrap();
// // this comment serves no purpose other than to link everything so i can collapse it in one click.
//     dbg!(details);
//     dbg!(state);
//     dbg!(timestamp_enabled);
//     dbg!(button_one_text);
//     dbg!(button_one_link);
//     dbg!(button_two_text);
//     dbg!(button_two_link);
//     dbg!(large_image_asset_name);
//     dbg!(large_image_asset_text);
//     dbg!(small_image_asset_name);
//     dbg!(small_image_asset_text);
}

fn main() {
    //
    // this code's only purpose is to make sure if setting the variables actually work
    // in this case, it did.
    // {
    //     let mut state = STATE.lock().unwrap();
    //     println!("initial: {}", state);
    //     *state = "a".to_string();
    //     println!("post edit test 1: {}", state);
    //     // rich_presence_callback();
    // }
    // let mut state = STATE.lock().unwrap();
    // println!("post edit test 2: {}", state);

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

// fn set_rich_presence() -> Result<(), Box<dyn std::error::Error>> {
fn set_rich_presence() -> Result<(), Box<dyn std::error::Error>> {
// fn set_rich_presence() {
// fn set_rich_presence(client_id: &str) {
    std::thread::spawn(|| {
        println!("starting rich presence");
        /*     oh my god rust is such a joke: the saga continues     */
        let client_id = CLIENT_ID.lock().unwrap();
        dbg!(&client_id);
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

        // let mut client = DiscordIpcClient::new("1173464118787846165").expect("wtf");
        let mut client = DiscordIpcClient::new(&client_id).expect("wtf");
        // let mut client = DiscordIpcClient::new("1173464118787846165")?;

        // let mut client_ref = client.as_ref();
        // client_ref = client_ref.expect("wtf").connect();

        // i dont even know what rust wants from me or my family anymore, wtf?????
        let _ = client.connect();

        let mut activity_base = activity::Activity::new();
        let mut activity_assets = activity::Assets::new();

        // this is where we init the buttons veclist thing.
        // N.B. we clone here because I don't want to mess up everything ahead.
        let mut buttons_vec_list_thing = activity_base.clone().buttons(vec![activity::Button::new("", "")]);

        if *details != "" {
            println!("details enabled, value is {}", &details);
            activity_base = activity_base.details(&details);
        }

        if *state != "" {
            println!("state enabled, value is {}", &state);
            activity_base = activity_base.state(&state);
        }

        if *timestamp_enabled == true {
            println!("timestamp enabled");
            activity_base = activity_base.timestamps(activity::Timestamps::new().start(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64));
        }

        // if *button_one_text != "" {
        //     if *button_one_link != "" {
        //         activity_base = activity_base.buttons(vec![activity::Button::new(&button_one_text, &button_one_link)])
        //     }
        // }

        // if *button_two_text != "" {
        //     if *button_two_link != "" {
        //         activity_base = activity_base.buttons(vec![activity::Button::new(&button_two_text, &button_two_link)])
        //     }
        // }

        if *button_one_text != "" {
            if *button_one_link != "" {
                println!("button one enabled; text: {}, link: {}", &button_one_text, &button_one_link);
                let mut tmp_is_using_btn_two = false;

                // TODO: fix indentation hell
                if *button_two_text != "" {
                    if *button_two_link != "" {
                        println!("button two enabled (alongside button one); text: {}, link: {}", &button_one_text, &button_one_link);
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
            println!("large image name: {}", &large_image_asset_name);
            activity_assets = activity_assets.large_image(&large_image_asset_name);

            if *large_image_asset_text != "" {
                println!("large image text: {}", &large_image_asset_text);
                activity_assets = activity_assets.large_text(&large_image_asset_text);
            }
        }

        if *small_image_asset_name != "" {
            println!("small image name: {}", &small_image_asset_name);
            activity_assets = activity_assets.small_image(&small_image_asset_name);

            if *small_image_asset_text != "" {
                println!("small image text: {}", &small_image_asset_text);
                activity_assets = activity_assets.small_text(&small_image_asset_text);
            }
        }

        println!("setting RPC");
        // you cant use ? for some reason, am i too stupid to figure this out?
        let _ = client.set_activity(activity_base.assets(activity_assets));

        loop {
            // let stop_req = MUTLITHREAD_ASSIST_REQUEST_STOP_RPC.lock().unwrap();
            // println!("{}", &stop_req);
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
