use std::str::FromStr as _;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue};

#[derive(Default, Clone)]
pub struct Headers {
    inner: HeaderMap,
    pub immutable: bool,
}

pub enum HeaderError {
    Immutable,
    InvalidHeader,
}

impl From<InvalidHeaderName> for HeaderError {
    fn from(_: InvalidHeaderName) -> Self {
        HeaderError::InvalidHeader
    }
}

impl From<InvalidHeaderValue> for HeaderError {
    fn from(_: InvalidHeaderValue) -> Self {
        HeaderError::InvalidHeader
    }
}

impl From<HeaderMap> for Headers {
    fn from(inner: HeaderMap) -> Self {
        Self {
            inner,
            immutable: false,
        }
    }
}

impl Headers {
    pub fn from_list(
        entries: impl IntoIterator<Item = (String, Vec<u8>)>,
    ) -> Result<Self, HeaderError> {
        let mut headers = HeaderMap::new();
        for (k, v) in entries {
            let name = HeaderName::from_str(&k)?;
            let value = HeaderValue::from_bytes(&v)?;
            headers.append(name, value);
        }
        Ok(Self {
            inner: headers,
            immutable: false,
        })
    }

    pub fn get(&self, name: impl AsRef<str>) -> Result<Vec<Vec<u8>>, HeaderError> {
        let name = HeaderName::from_str(name.as_ref())?;
        let values = self
            .inner
            .get_all(name)
            .iter()
            .map(|x| x.as_bytes().to_vec())
            .collect();
        Ok(values)
    }

    pub fn has(&self, name: impl AsRef<str>) -> bool {
        let Ok(name) = HeaderName::from_str(name.as_ref()) else {
            return false;
        };
        self.inner.contains_key(name)
    }

    pub fn set(
        &mut self,
        name: impl AsRef<str>,
        values: impl IntoIterator<Item = impl AsRef<[u8]>>,
    ) -> Result<(), HeaderError> {
        if self.immutable {
            return Err(HeaderError::Immutable);
        }
        let name = HeaderName::from_str(name.as_ref())?;
        let values = values
            .into_iter()
            .map(|val| HeaderValue::from_bytes(val.as_ref()))
            .collect::<Result<Vec<_>, _>>()?;
        self.inner.remove(&name);
        for val in values {
            self.inner.append(&name, val);
        }
        Ok(())
    }

    pub fn delete(&mut self, name: impl AsRef<str>) -> Result<(), HeaderError> {
        if self.immutable {
            return Err(HeaderError::Immutable);
        }
        let name = HeaderName::from_str(name.as_ref())?;
        self.inner.remove(name);
        Ok(())
    }

    pub fn append(
        &mut self,
        name: impl AsRef<str>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), HeaderError> {
        if self.immutable {
            return Err(HeaderError::Immutable);
        }
        let name = HeaderName::from_str(name.as_ref())?;
        let value = HeaderValue::from_bytes(value.as_ref())?;
        self.inner.append(name, value);
        Ok(())
    }

    pub fn entries(&self) -> Vec<(String, Vec<u8>)> {
        self.inner
            .iter()
            .map(|(k, v)| (k.as_str().into(), v.as_bytes().to_vec()))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
