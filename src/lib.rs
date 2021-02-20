mod geom;

#[cfg(test)]
mod tests {
    use crate::geom::OldFile;

    #[test]
    fn it_works() {
        let file = OldFile::read("TU204Z.DAT");
        println!("{:?}", file)
    }
}

