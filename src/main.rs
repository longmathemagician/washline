mod rope_mod;

use std::env;
use std::fs;

use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::Label;

fn text_widget(source: String) -> impl Widget<()> {
	Label::new(source)
}


fn main() -> Result<(), PlatformError> {
	let args: Vec<String> = env::args().collect();

	let mut rope_instance = rope_mod::Rope::new();
	let leaf_length: usize = 32;

	let mut file_name = &"test_document.txt".to_string();
	if args.len() > 1 {
		file_name = &args[1];
	}
	let file_path = std::path::Path::new(file_name);

	let file: String = fs::read_to_string(file_path).expect("Error loading file.");

	let file_length = file.len();
	let mut file_index: usize = 0;

	while file_index < file_length {
		rope_instance.add_branch(file.chars().take(file_index + 2 * leaf_length).skip(file_index).collect());
		file_index += 2*leaf_length;
	}

	rope_instance.rebuild();
	let source = rope_instance.get_text();
	// println!("{}", source);

	AppLauncher::with_window(WindowDesc::new(text_widget(source))).launch(())?;
    Ok(())
}
