use std::{
    path::PathBuf,
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::Duration,
};
slint::include_modules!();

use chrono::{Local, NaiveTime, TimeDelta, Timelike};

use system_status_bar_macos::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

//TODO: Structure to be serialized into JSON or something else
struct AppSettings {
    pub trigger_time: NaiveTime,
    pub countdown_seconds: u32,
    pub apps_to_quit: Vec<(String, PathBuf)>,
}

struct TriggerCountdownUITime {
    //TODO: make sure to add a daily_trigger_time and a current_trigger_time
    //This would replace `repeat_duration_minutes``
    pub time: NaiveTime,
    repeat_duration_minutes: Option<i64>,
}

impl TriggerCountdownUITime {
    /// Creates a new trigger time from hours and minutes
    pub fn at(hour: u32, minute: u32) -> Self {
        let time = NaiveTime::from_hms_opt(hour, minute, 00).unwrap();
        Self {
            time,
            repeat_duration_minutes: None,
        }
    }

    /// Returns duration to sleep from current time until next trigger time.
    /// UI triggers once per day with this setup
    pub fn sleep_duration_from_now(&mut self) -> Duration {
        let now = Local::now().naive_local().time();
        let mut time_difference = self.time - now;

        println!("time different: {}", time_difference);
        //FIX: this logic is not very good. Need a better logic to trigger every X amount of time.
        if time_difference.num_milliseconds() < 0 {
            let time_to_repeat = match self.repeat_duration_minutes {
                Some(minutes) => TimeDelta::minutes(minutes),
                None => TimeDelta::hours(24),
            };

            time_difference = time_difference + time_to_repeat;

            self.time += time_difference;
        }

        let duration = time_difference.abs().to_std().unwrap();
        println!("duration to next trigger time: {:?}", duration);

        duration
    }

    pub fn repeat_in(&mut self, minutes: i64) {
        self.repeat_duration_minutes = Some(minutes);
    }

    /// Checks current time against time to trigger UI
    pub fn is_now(&self) -> bool {
        let now = Local::now().naive_local().time();
        let result = now > self.time;
        println!("trigger time is now?: {}", result);

        result
    }
}

fn main() {
    //TODO: add this to a basic struct
    let time_now = Local::now().naive_local().time();
    // Responsible to trigger UI countdown
    let mut trigger_time = TriggerCountdownUITime::at(time_now.hour(), time_now.minute() + 1);
    // Will trigger every 1 minute, for testing purposes at the moment.
    trigger_time.repeat_in(1);

    let (sender, _receiver) = channel::<bool>();
    let sender_run_clone = sender.clone();
    let sender_main_clone = sender.clone();

    //TODO: way to create "multiple windows" is to use the same component
    // let a = AppWindow::new().unwrap();
    // a.show().unwrap();
    let ui = AppWindow::new().unwrap();

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
                    ui.set_counter(10);
                }
            });
        }
    });

    //TODO: create a menu that has settings to setup time, message and programs to close
    let _status_item = StatusItem::new(
        "☀️",
        Menu::new(vec![
            MenuItem::new(format!("version: {}", VERSION), None, None),
            //TODO remove this item unless there's a reason we would like to trigger the
            // UI manually
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

    spawn(move || {
        loop {
            // REVIEW: the loop code runs every `sleep_duration_from_now`
            // because currently it only triggers UI countdown
            // However, if we want to check other processes in this thread that are not
            // tied only to the UI countdown, we should refactor all this of this code.
            sleep(trigger_time.sleep_duration_from_now());
            // let value = receiver.recv().unwrap();

            let time_now = Local::now().naive_local().time();
            println!("current time: {}", time_now.to_string());

            if trigger_time.is_now() {
                println!("Showing Countdown UI!");
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
