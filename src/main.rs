use std::convert::TryInto;

use slint::{Timer, TimerMode};
slint::include_modules!();

fn main() {

    let application = Flow::new().unwrap();
	let weak_minutes = application.as_weak();
	let weak_seconds = application.as_weak();

	let timer = Timer::default();
	{
		let application_weak = application.as_weak();
		
		timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1000), move || {
			let application = application_weak.unwrap();
			application.invoke_tick(1000);
		});
	}
	
	application.on_sendMinutes(move |value| {
		let result = convert_string_to_integer(value.as_str());
		
		let application = weak_minutes.unwrap();
		application.set_backup_minutes(result);
		application.invoke_reset();
	});
	
	application.on_sendSeconds(move |value| {
		let result = convert_string_to_integer(value.as_str());
		
		let application = weak_seconds.unwrap();
		application.set_backup_seconds(result);
		application.invoke_reset();
	});

    application.run().unwrap();

}

fn convert_string_to_integer(value: &str) -> i32 {
	let mut new_value = value;
	let mut its_number = true;
	let mut i = 1;
	let mut result = 0;
	
	// Check if the inserted value is a number
	while new_value.len() > 0 && its_number {
		let ch = new_value.chars().next().unwrap();
		new_value = &new_value[1..new_value.len()];
		
		let ch_num = ch.to_digit(10).unwrap_or(11) as i32;
		
		if ch_num == 11 {
			its_number = false;
		} else {
			let exp: u32 = (value.len() - i).try_into().unwrap();
			
			result += ch_num * 10_i32.pow(exp);
		}
		
		i += 1;
	}
	
	return result;
}