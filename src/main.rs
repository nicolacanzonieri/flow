use slint::{Timer, TimerMode};
slint::include_modules!();

fn main() {

    let application = Flow::new().unwrap();

	let timer = Timer::default();
	{
		let application_weak = application.as_weak();
		
		timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1000), move || {
			let application = application_weak.unwrap();
			application.invoke_tick(1000);
		});
	}

    application.run().unwrap();

}
