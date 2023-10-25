use iri_s::{IriS, IriSError};
use prefixmap::{PrefixMap, PrefixMapError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DerefError {

    #[error(transparent)]
    IriSError(#[from] IriSError),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),
    
    #[error("No prefix map to dereference prefixed name {prefix}{local}")]
    NoPrefixMapPrefixedName { prefix: String, local: String }
}

pub trait Deref {
    fn deref(&self, 
        base: &Option<IriS>, 
        prefixmap: &Option<PrefixMap>
    ) -> Result<Self, DerefError> where Self: Sized; 

    fn deref_opt<T>(
        maybe: &Option<T>, 
        base: &Option<IriS>, 
        prefixmap: &Option<PrefixMap>) -> Result<Option<T>, DerefError> where T: Deref {
       let result = match maybe {
        None => None,
        Some(t) => {
           let new_t = t.deref(base, prefixmap)?;
           Some(new_t)
        }
       };
        Ok(result)
    }

    fn deref_box<T>(
        bt: &Box<T>, 
        base: &Option<IriS>, 
        prefixmap: &Option<PrefixMap>) -> Result<Box<T>, DerefError> where T: Deref {
        let t = bt.as_ref().deref(base, prefixmap)?;
        Ok(Box::new(t))
    }

    fn deref_opt_box<T>(
        maybe: &Option<Box<T>>, 
        base: &Option<IriS>, 
        prefixmap: &Option<PrefixMap>) -> Result<Option<Box<T>>, DerefError> where T: Deref {
       let result = match maybe {
        None => None,
        Some(t) => {
           let new_t = t.deref(base, prefixmap)?;
           Some(Box::new(new_t))
        }
       };
       Ok(result)
    }

    fn deref_vec<T>(ts: &Vec<T>,         
        base: &Option<IriS>, 
        prefixmap: &Option<PrefixMap>) -> Result<Vec<T>, DerefError> where T: Deref {
            let mut new_ts = Vec::new();
            for t in ts {
              new_ts.push(t.deref(base, prefixmap)?);
            }
            Ok(new_ts)     
    }


    fn deref_opt_vec<T>(
        maybe_ts: &Option<Vec<T>>, 
        base: &Option<IriS>, 
        prefixmap: &Option<PrefixMap>) -> Result<Option<Vec<T>>, DerefError> where T: Deref {
       let result = match maybe_ts {
        None => None,
        Some(ts) => {
           let mut new_ts = Vec::new();
           for t in ts {
            new_ts.push(t.deref(base, prefixmap)?);
           }
           Some(new_ts)
        }
       };
        Ok(result)
    }

}

