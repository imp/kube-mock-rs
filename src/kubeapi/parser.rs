use core::str;

use kube::discovery::ApiGroup;

use super::*;

#[derive(Debug)]
pub struct ParsedResource {
    api_version: String,
    plural: String,
    namespace: Option<String>,
    name: Option<String>,
}

impl ParsedResource {
    fn new<'a>(
        group: &str,
        version: &str,
        plural: &str,
        namespace: impl Into<Option<&'a str>>,
        name: impl Into<Option<&'a str>>,
    ) -> Self {
        let api_version = if group.is_empty() {
            version.to_string()
        } else {
            format!("{group}/{version}")
        };
        let plural = plural.to_string();
        let namespace = namespace.into().map(ToString::to_string);
        let name = name.into().map(ToString::to_string);
        Self {
            api_version,
            plural,
            namespace,
            name,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn plural(&self) -> &str {
        &self.plural
    }
}

impl str::FromStr for ParsedResource {
    type Err = kube::Error;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        parse_path(path).map_err(|_err| kube::Error::LinesCodecMaxLineLengthExceeded)
    }
}

fn parse_path(path: &str) -> Result<ParsedResource, Error> {
    if let Some((_api, rest)) = path.split_once("/api/") {
        parse_version(ApiGroup::CORE_GROUP, rest)
    } else if let Some((_apis, rest)) = path.split_once("/apis/") {
        let (group, rest) = rest.split_once("/").ok_or(Error::WrongPath)?;
        parse_version(group, rest)
    } else {
        Err(Error::WrongPrefix)
    }
}

fn parse_version(group: &str, path: &str) -> Result<ParsedResource, Error> {
    let mut path = path.split("/");

    let version = path.next().ok_or(Error::WrongVersion)?;
    let plural = path.next().ok_or(Error::WrongPath)?;
    let name_or_namespace = path.next();
    let kind = path.next();
    let name = path.next();
    match (plural, name_or_namespace, kind) {
        (plural, None, None) => Ok(ParsedResource::new(group, version, plural, None, name)),
        (plural, Some(name), None) => Ok(ParsedResource::new(group, version, plural, None, name)),
        ("namespaces", Some(namespace), Some(plural)) => {
            Ok(ParsedResource::new(group, version, plural, namespace, name))
        }
        _ => Err(Error::WrongPath),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Wrong Prefix")]
    WrongPrefix,
    #[error("Wrong Version")]
    WrongVersion,
    #[error("Wrong Path")]
    WrongPath,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_leading_slash() {
        assert_eq!(parse_path("something").unwrap_err(), Error::WrongPrefix);
    }
}
