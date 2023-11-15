// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
use druid::widget::Label;
use druid::{AppLauncher, Widget, WindowDesc, Data, Env};
use std::string::String;
use std::thread;
use core::time::Duration;


#[derive(Clone, Data)]
struct AppData {
    main_test_label: String
}

fn build_ui() -> impl Widget<AppData> {
    Label::new(|data: &AppData, _: &Env| data.main_test_label.to_string())
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Tough Cookie");
    let initial_data = AppData{main_test_label: "".to_string()};
    let launcher = AppLauncher::with_window(main_window);

    // If we want to create commands from another thread `launcher.get_external_handle()`
    // should be used. For sending commands from within widgets you can always call
    // `ctx.submit_command`
    let event_sink = launcher.get_external_handle();
    // We create a new thread and generate colours in it.
    // This happens on a second thread so that we can run the UI in the
    // main thread. Generating some colours nicely follows the pattern for what
    // should be done like this: generating something over time
    // (like this or reacting to external events), or something that takes a
    // long time and shouldn't block main UI updates.
    thread::spawn(move || main_thread(event_sink));

    launcher
        .launch(initial_data)
        .expect("Failed to launch application (it's your lucky day)");
}

fn main_thread(event_sink: druid::ExtEventSink) {
    let final_label = "You should improve your security <3";

    // Initializing the window to prevent characters from not being displayed
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.main_test_label.push(' ');
    });
    std::thread::sleep(Duration::from_millis(1000));
    
    for chars in final_label.chars() {
        // Add the character
        event_sink.add_idle_callback(move |data: &mut AppData| {
            data.main_test_label.push(chars);
        });


        // Wait a bit
        std::thread::sleep(Duration::from_millis(100));
    }
}