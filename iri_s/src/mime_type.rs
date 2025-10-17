/// Types that implement this trait can inform about their MIME-TYPE
/// More information about MIME types:
/// https://www.iana.org/assignments/media-types/media-types.xhtml
pub trait MimeType {
    fn mime_type(&self) -> &'static str;
}
