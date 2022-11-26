pub(crate) trait StringExt {
    fn escape(&self) -> String;
    fn quote(&self) -> String;
    fn deflate(&self) -> Vec<u8>;
}

pub(crate) trait BytesExt {
    fn base64(&self) -> String;
}

impl StringExt for String {
    fn escape(&self) -> String {
        self.replace("\"", "\\\"")
    }

    fn quote(&self) -> String {
        format!("\"{}\"", self.escape())
    }

    fn deflate(&self) -> Vec<u8> {
        use deflate::write::ZlibEncoder;
        use deflate::Compression;
        use std::io::Write;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
        encoder.write_all(self.as_bytes()).expect("Write error");
        encoder.finish().expect("Failed to finish compression")
    }
}

impl BytesExt for Vec<u8> {
    fn base64(&self) -> String {
        use std::io::Write;
        let mut enc = base64::write::EncoderStringWriter::new(base64::URL_SAFE);
        enc.write_all(self).unwrap();
        enc.into_inner()
    }
}
