mod deprecated;
use crate::deprecated::AeflotInput;
use std::error::Error;
use std::fs;


pub fn convert_aeflot_dep_2_json(path: &str) -> Result<String, Box<dyn Error>> {
    let file = AeflotInput::read(path)?;
    let path = String::from(path);
    let mut path: String = path.chars()
        .into_iter()
        .take(path.rfind('.').expect("unknown file extension"))
        .collect();
    path.push_str(".json");
    file.write_json(&path)?;
    Ok(path)
}

pub fn convert_json_2_aeflot_dep(path: &str) -> Result<String, Box<dyn Error>> {
    let file = AeflotInput::read_json(path)?;
    let path = String::from(path);
    let mut path: String = path.chars()
        .into_iter()
        .take(path.rfind('.').expect("unknown file extension"))
        .collect();
    path.push_str(".DAT");
    file.write(&path)?;
    Ok(path)
}

pub fn convert_json_s_2_aeflot_dep_s(string: &str) -> Result<String, Box<dyn Error>> {
    let file = AeflotInput::from_json_string(&string)?;
    Ok(file.to_string())
}

pub fn convert_aeflot_dep_s_2_json_s(string: &str) -> Result<String, Box<dyn Error>> {
    fs::write("aeflot.tmp", string)?;
    let file = AeflotInput::read("aeflot.tmp")?;
    fs::remove_file("aeflot.tmp")?;
    Ok(file.to_json_string())
}

pub fn create_json_template(path: &str) -> Result<(), Box<dyn Error>> {
    let file = AeflotInput::default();
    file.write_json(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::deprecated::AeflotInput;

    #[test]
    fn it_works() {
        let file = AeflotInput::read("test_files/input/TMK.dat").unwrap();
        println!("{:?}", file);
        file.write("test_files/test.txt").unwrap();
        file.write_json("test_files/test.json").unwrap()
    }
}
