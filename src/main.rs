slint::include_modules!();

fn main() {
    let app = Application::new().unwrap();

    app.on_buttonPressed(move |value| {
        match value.as_str() {
            "play" => {
                println!("Play!");
            },

            "stop" => {
                println!("Stop!");
            },

            "pause" => {
                println!("Pause!");
            },

            "settings" => {
                println!("Settings");
            },

            _ => {
                // nothing
            }
        }
    });

    app.run().unwrap();
}
