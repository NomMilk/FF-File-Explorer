use std::fs::File;
use std::fs;

pub fn createfile(_textname: &str)
{
	let mut _file = File::create(format!("{}.txt", _textname.trim())); 
}

pub fn getfiles() -> Vec<String>
{
    let _paths = fs::read_dir("./").unwrap();
	let mut _returnpaths: Vec<String>  = Vec::new(); 

    for path in _paths {
		_returnpaths.push(path.unwrap().path().display().to_string());
    }
	
	_returnpaths
}

