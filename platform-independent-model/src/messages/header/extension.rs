use crate::messages::ByteOrder;
use chrono::{DateTime, Utc};

/// An optional extension to a [`Message`](super::super::Message)
#[derive(Debug, Default)]
pub struct Extension {
    endianess: ByteOrder,
    length: Option<usize>,
    sent_timestamp: Option<DateTime<Utc>>,
    checksum: Option<Checksum>,
    parameters: Vec<Parameter>,
}

impl Extension {
    /// Use the builder to configure the header extension
    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
#[must_use]
pub struct Builder {
    extension: Extension,
}

impl Builder {
    /// Set the endianess of the data payload
    pub fn endianess(mut self, endianess: ByteOrder) -> Self {
        self.extension.endianess = endianess;
        self
    }

    pub fn length(mut self, len: usize) -> Self {
        self.extension.length = Some(len);
        self
    }

    pub fn sent_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.extension.sent_timestamp = Some(timestamp);
        self
    }

    pub fn checksum(mut self, checksum: Checksum) -> Self {
        self.extension.checksum = Some(checksum);
        self
    }

    pub fn parameter(mut self, param: Parameter) -> Self {
        self.extension.parameters.push(param);
        self
    }

    pub fn build(self) -> Extension {
        self.extension
    }
}

#[derive(Debug)]
pub struct Checksum;

#[derive(Debug)]
pub struct Parameter;
