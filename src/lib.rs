use std::{fs::File, error::Error, io::{BufReader, Read}};

trait FileCompareExt {
    fn same_as(&self, other: &File) -> Result<bool, Box<dyn Error>>;
    fn same_as_buf_size(&self, other: &File, buff_size: usize) -> Result<bool, Box<dyn Error>>;
}

impl FileCompareExt for File {
    fn same_as(&self, other: &File) -> Result<bool, Box<dyn Error>> {
        self.same_as_buf_size(other, 8192)
    }
    fn same_as_buf_size(&self, other: &File, buff_size: usize) -> Result<bool, Box<dyn Error>> {
        if self
            .metadata()
            .and_then(|md_a| other.metadata().map(|md_b| md_a.len() == md_b.len()))
            .unwrap_or(true)
        // Treat unknown size as same size
        {
            let mut a = BufReader::new(self);
            let mut b = BufReader::new(other);
            let mut buf_a = vec![0; buff_size].into_boxed_slice();
            let mut buf_b = vec![0; buff_size].into_boxed_slice();
            while a.read(&mut buf_a)? > 0 {
                let _ = b.read(&mut buf_b)?;
                if *buf_a != *buf_b {
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            // Files are different size, so we know they differ without needing to read from them
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
