/// Types that implement this trait can inform about their MIME-TYPE
/// More information about MIME types:
/// https://www.iana.org/assignments/media-types/media-types.xhtml
// TODO - move to another crate?
pub trait MimeType {
    fn mime_type(&self) -> &'static str;
}
