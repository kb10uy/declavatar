use serde::Serialize;
use serde_json::Error as SerdeJsonError;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Jsoned<T> {
    data: T,
    serialized: Option<String>,
}

#[allow(dead_code)]
impl<T: Serialize> Jsoned<T> {
    pub fn new(data: T) -> Result<Jsoned<T>, SerdeJsonError> {
        let serialized = Some(serde_json::to_string(&data)?);
        Ok(Jsoned { data, serialized })
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn json(&self) -> Option<&str> {
        self.serialized.as_deref()
    }

    pub fn update(&mut self, f: impl FnOnce(&mut T)) -> Result<(), SerdeJsonError> {
        f(&mut self.data);
        self.serialize()?;
        Ok(())
    }

    pub fn serialize(&mut self) -> Result<(), SerdeJsonError> {
        self.serialized = Some(serde_json::to_string(&self.data)?);
        Ok(())
    }
}
