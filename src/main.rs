use iced::widget::{container, button, row, text_input, Column, Text};
use iced::Element;

mod filemanage;

struct FileEditor
{
	currentfiles: Vec<String>,
	filename: String,
}

#[derive(Debug, Clone)]
enum Message
{
	UpdateFiles,
	AddFilesName(String),
	AddFiles,
	DestroyFiles
}

impl Default for FileEditor
{
    fn default() -> Self
	{
        FileEditor
		{
            currentfiles: filemanage::getfiles(),
			filename: String::new(),
        }
    }
}

impl FileEditor
{
	fn update(&mut self, message: Message)
	{
		match message
		{	
			Message::UpdateFiles =>
			{
				self.currentfiles = filemanage::getfiles();
			}

			Message::AddFiles =>
			{
				filemanage::createfile(&self.filename);
				self.currentfiles = filemanage::getfiles();
			}

			Message::AddFilesName(_name) =>
			{
				//filemanage::createfile(&self.filename);
				//self.currentfiles = filemanage::getfiles();
				self.filename = _name
			}

			Message::DestroyFiles =>
			{
				todo!();
			}
		}
	}

	fn view(&self) -> Element<'_, Message>
	{
		let mut _textelements: Column<Message> = Column::new();

		_textelements = _textelements.push(button("Update Files").on_press(Message::UpdateFiles));

		_textelements = _textelements.push
			(
				row![	
		 				button("Create Files").on_press(Message::AddFiles),

						text_input("Type filename here...", &self.filename)
						.on_input(Message::AddFilesName),
					]
			);

		for _file in &self.currentfiles
		{
			_textelements = _textelements.push(Text::new(_file));
		}
		
		container(_textelements)
			.padding(30)
			.into()
	}
}

fn main() -> iced::Result
{
	filemanage::getfiles();

	iced::run("Fish's FileEditor", FileEditor::update, FileEditor::view)
}
