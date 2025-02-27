use iced::widget::{button, column, container, text, Column, Text};
use iced::Element;

mod filemanage;

#[derive(Default)]
struct FileEditor {}

#[derive(Debug, Clone, Copy)]
enum Message
{
	UpdateFiles,
	AddFiles,
	DestroyFiles,
}

impl FileEditor
{
	fn update(&mut self, message: Message)
	{
		match message
		{	
			Message::UpdateFiles =>
			{
				filemanage::getfiles();
			}

			Message::AddFiles =>
			{
				todo!();
				//filemanage::createfile("file");
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

		for _file in filemanage::getfiles()
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
