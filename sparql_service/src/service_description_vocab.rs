use const_format::concatcp;
use iri_s::IriS;
use lazy_static::lazy_static;

pub const SD_STR: &str = "http://www.w3.org/ns/sparql-service-description#";
pub const SD_SERVICE_STR: &str = concatcp!(SD_STR, "Service");
pub const SD_ENDPOINT_STR: &str = concatcp!(SD_STR, "endpoint");

lazy_static! {
    pub static ref SD: IriS = IriS::new_unchecked(SD_STR);
    pub static ref SD_SERVICE: IriS = IriS::new_unchecked(SD_SERVICE_STR);
    pub static ref SD_ENDPOINT: IriS = IriS::new_unchecked(SD_ENDPOINT_STR);
}
