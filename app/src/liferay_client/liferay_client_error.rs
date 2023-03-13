use std::{
    error::{self, Error},
    fmt::{Debug, Display},
};

use crate::util::classify_serde_error;

#[derive(Debug)]
pub enum LiferayClientError<'a, T> {
    Serialization {
        type_name: &'a str,
        origin: serde_json::Error,
    },
    Request {
        origin: batch_api::reqwest::Error,
    },
    Io {
        origin: std::io::Error,
    },
    Response {
        origin: object_admin::apis::ResponseContent<T>,
    },
    // InvalidPayload {
    //     message: String,
    // },
}

impl<'a, T> Display for LiferayClientError<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialization { type_name, origin } => {
                write!(
                    f,
                    "Failed to serialize object: {type_name}. Error type: {}",
                    classify_serde_error(origin)
                )
            }
            Self::Request { origin } => write!(f, "Request failed: {origin}"),
            Self::Io { .. } => write!(f, "Io error encounted"),
            Self::Response { origin } => {
                write!(
                    f,
                    "Server responded with error. Status: {}, content: {}",
                    origin.status, origin.content
                )
            } // Self::InvalidPayload { message } => write!(f, "Invalid payload: {message}",),
        }
    }
}

impl<'a, T> Error for LiferayClientError<'a, T>
where
    T: Debug,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Serialization { origin, .. } => Some(origin),
            Self::Request { origin } => Some(origin),
            Self::Io { origin } => Some(origin),
            Self::Response { .. } => None,
            // Self::InvalidPayload { .. } => None,
        }
    }
}

impl<'a, T> From<object_admin::apis::Error<T>> for LiferayClientError<'a, T> {
    fn from(value: object_admin::apis::Error<T>) -> Self {
        match value {
            object_admin::apis::Error::Reqwest(e) => e.into(),
            object_admin::apis::Error::Serde(e) => Self::Serialization {
                type_name: classify_serde_error(&e),
                origin: e,
            },
            object_admin::apis::Error::Io(e) => Self::Io { origin: e },
            object_admin::apis::Error::ResponseError(e) => Self::Response { origin: e },
        }
    }
}

impl<'a, T> From<batch_api::reqwest::Error> for LiferayClientError<'a, T> {
    fn from(value: batch_api::reqwest::Error) -> Self {
        Self::Request { origin: value }
    }
}

impl<'a, T> From<serde_json::Error> for LiferayClientError<'a, T> {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization {
            type_name: classify_serde_error(&value),
            origin: value,
        }
    }
}
