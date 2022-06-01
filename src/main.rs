use std::{env, fs};

use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

// Import the rope module
mod rope_mod;

// Create a very simple widget to display the rope data
fn text_widget(source: String) -> impl Widget<()> {
	Label::new(source)
}

fn main() -> Result<(), PlatformError> {
	// Collect potential launch arguments
	let args: Vec<String> = env::args().collect();

	// Create a path to the first launch argument, or to a test file if
	// launch args were not provided
	let mut file_name = &"test_document.txt".to_string();
	if args.len() > 1 {
		file_name = &args[1];
	}
	let file_path = std::path::Path::new(file_name);

	// Load the file into a string
	let file: String = fs::read_to_string(file_path).expect("File load error");

	// Create a rope instance, and fill it with text from the file
	let mut rope_instance = rope_mod::Rope::new();
	let leaf_length: usize = 64; // Each leaf will hold 64 UTF-8 graphemes
	let file_length = file.len();
	let mut file_index: usize = 0;
	while file_index < file_length {
		rope_instance.add_string(
			file.chars()
				.take(file_index + 2 * leaf_length)
				.skip(file_index)
				.collect()
			);
		file_index += 2*leaf_length;
	}

	// Rebuild the rope
	// Handy for now as the above code generates a lot of empty leaves
	rope_instance.rebuild();

	// Collect all of the text from the rope and print it to the console
	let source = rope_instance.get_text();
	println!("{}", source);

	// Collect a subset of the text in the rope and print it to the console
	let substr = rope_instance.substring(0,rope_instance.get_length()-1);
	println!("{}", substr.unwrap_or_default());

	// Open a new window, displaying the data stored inside the rope instance
	AppLauncher::with_window(WindowDesc::new(text_widget(source))).launch(())?;
	Ok(())
}
