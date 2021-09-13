mod tests {
    use keepass::*;
    use std::io::Write;
    use std::{fs::File, path::Path};

    #[test]
    fn dump_kdbx3() -> Result<()> {
        let path = Path::new("tests/resources/test_db_with_password.kdbx");
        let db = Database::open(&mut File::open(path)?, Some("demopass"), None)?;
        let res = db.dump(Some("demopass"), None)?;
        println!("{}", String::from_utf8(res).unwrap());
        Ok(())
    }
    #[test]
    fn dump_kdbx4() -> Result<()> {
        let path = Path::new("tests/resources/test_db_kdbx4_with_password_aes.kdbx");
        let db = Database::open(&mut File::open(path)?, Some("demopass"), None)?;
        let res = db.dump(Some("demopass"), None)?;
        let mut f = std::fs::File::create("kdbx4-dump.bin")?;
        f.write_all(&res)?;
        Ok(())
    }
}
