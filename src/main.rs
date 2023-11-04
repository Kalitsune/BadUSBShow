use druid::widget::Label;
use druid::{AppLauncher, Widget, WindowDesc, Data, Env};
use std::String;

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

#[derive(Clone, Data)]
struct AppData {
    main_test_label: String
}

fn build_ui() -> impl Widget<AppData> {
    Label::new(|data: &AppData, _: &Env| &AppData::main_test_label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Tough Cookie");
    let initial_data = AppData(main_test_label: "");
    let launcher = AppLauncher::with_window(main_window)

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
    let mut label = String::new();
    let final_label = "You should improve your security <3";

    loop {
        //check if both labels have the same lenght
        if label.len() >= final_label.len() {
            break;
        } else {
            //add the next character
            label += final_label[label.len()+1];
        }

        // schedule idle callback to change the data
        event_sink.add_idle_callback(move |data: &mut AppData| {
            *data.main_test_label = label;
        });

        //wait a bit
        thread::sleep(Duration::from_millis(1*1000));
    }
}