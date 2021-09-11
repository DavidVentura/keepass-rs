mod tests {
    use keepass::*;
    use std::{fs::File, path::Path};

    #[test]
    fn dump_kdbx3() -> Result<()> {
        let path = Path::new("tests/resources/test_db_with_password.kdbx");
        let db = Database::open(&mut File::open(path)?, Some("demopass"), None)?;
        let res = db.dump()?;
        println!("{}", String::from_utf8(res).unwrap());
        Ok(())
    }
}
