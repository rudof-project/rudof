use crate::data_format::DataFormat;

pub trait MimeType {
    fn mime_type(&self) -> String;
}
