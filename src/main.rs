use std::{
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::Duration,
};
slint::include_modules!();

use chrono::{Local, NaiveTime, Timelike};

use system_status_bar_macos::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let ui = AppWindow::new().unwrap();

    // Date to trigger
    //TODO: add this to a basic struct
    let time_now = Local::now().naive_local().time();
    let trigger_data_low =
        NaiveTime::from_hms_opt(time_now.hour(), time_now.minute() + 1, 00).unwrap();
    let trigger_data_high =
        NaiveTime::from_hms_opt(time_now.hour(), time_now.minute() + 1, 59).unwrap();

    let time_diff = trigger_data_low - time_now;
    let sleep_duration = time_diff.to_std().unwrap();

    println!("Sleep duration: {:?}", sleep_duration);

    let (sender, _receiver) = channel::<bool>();
    let sender_run_clone = sender.clone();
    let sender_main_clone = sender.clone();

    ui.on_countdown_timer({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            //TODO improvement: replace this with a single timer instead of creating multiple ones
            slint::Timer::single_shot(Duration::from_secs(1), move || {
                if ui.get_counter() > 0 && ui.window().is_visible() {
                    ui.set_counter(ui.get_counter() - 1);
                    println!("Timer countdown: {}", ui.get_counter());
                    // create a new timer again
                    ui.invoke_countdown_timer();
                } else {
                    println!("Countdown finished!");
                    ui.hide().unwrap();
                }
            });
        }
    });

    //TODO: create a menu that has settings to setup time, message and programs to close
    let _status_item = StatusItem::new(
        "☀️",
        Menu::new(vec![
            MenuItem::new(format!("version: {}", VERSION), None, None),
            //TODO remove this item unless there's a reason we would like to trigger the UI manually
            MenuItem::new(
                "Run",
                Some(Box::new(move || {
                    let _ = sender_run_clone.send(true);
                })),
                None,
            ),
            MenuItem::new(
                "Settings",
                Some(Box::new(|| {
                    println!("clicked!");
                })),
                None,
            ),
        ]),
    );

    let ui_handle = ui.as_weak();
    let mut is_countdown_running = false;

    spawn(move || {
        loop {
            sleep(sleep_duration);
            // let value = receiver.recv().unwrap();

            let mut launch_countdown = false;
            let time_now = Local::now().naive_local().time();

            println!("current time: {}", time_now.to_string());
            if time_now > trigger_data_low && time_now < trigger_data_high {
                launch_countdown = true;
            }

            if launch_countdown && !is_countdown_running {
                println!("Showing UI!");
                is_countdown_running = true;
                let ui_handle_copy = ui_handle.clone();
                let sender_clone = sender_main_clone.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    ui_handle_copy.unwrap().invoke_countdown_timer();
                    ui_handle_copy.unwrap().show().unwrap();
                    sender_clone.send(false).unwrap();
                });
            }
            // terminator.terminate();
        }
    });

    sender.send(false).unwrap();

    // The UI event loop needs to run on the main thread. We can then invoke events in this loop
    // from different threads; e.g. show UI, etc
    slint::run_event_loop_until_quit().unwrap();
}
