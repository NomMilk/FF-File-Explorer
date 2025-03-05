use iced::widget::{container, button, row, text_input, Column, Text};
use iced::Element;

mod filemanage;

struct FileEditor
{
	_currentfiles: Vec<String>,
	_filename: String,
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
            _currentfiles: filemanage::getfiles(),
			_filename: String::new(),
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
				self._currentfiles = filemanage::getfiles();
			}

			Message::AddFiles =>
			{
				filemanage::createfile(&self._filename);
				self._currentfiles = filemanage::getfiles();
			}

			Message::AddFilesName(_name) =>
			{
				//filemanage::createfile(&self._filename);
				//self._currentfiles = filemanage::getfiles();
				self._filename = _name
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
		_textelements = _textelements.push(button("Create Files").on_press(Message::AddFiles));

		_textelements = _textelements.push(
		text_input("Type filename here...", &self._filename)
        	.on_input(Message::AddFilesName));

		for _file in &self._currentfiles
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
