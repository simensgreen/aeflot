mod deprecated;

#[cfg(test)]
mod tests {
    use crate::deprecated::AeflotInput;

    #[test]
    fn it_works() {
        let file = AeflotInput::read("TU204Z.DAT");
        println!("{:?}", file);
        file.write("test.txt");
    }
}
