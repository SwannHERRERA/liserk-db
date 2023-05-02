use bson::Uuid;

use crate::errors::Error;

pub fn to_uuid<S: AsRef<str>>(id: S) -> Result<Uuid, Error> {
    Uuid::parse_str(id.as_ref()).map_err(|_| Error::ParseUuid(id.as_ref().to_string()))
}