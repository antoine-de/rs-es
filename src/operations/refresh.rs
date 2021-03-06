/*
 * Copyright 2016 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Refresh an Index

use hyper::status::StatusCode;

use ::{Client, EsResponse};
use ::error::EsError;

use super::{format_multi, ShardCountResult};

pub struct RefreshOperation<'a, 'b> {
    /// The HTTP client
    client: &'a mut Client,

    /// The indexes being refreshed
    indexes: &'b [&'b str]
}

impl<'a, 'b> RefreshOperation<'a, 'b> {
    pub fn new(client: &'a mut Client) -> RefreshOperation {
        RefreshOperation {
            client:  client,
            indexes: &[]
        }
    }

    pub fn with_indexes(&'b mut self, indexes: &'b [&'b str]) -> &'b mut Self {
        self.indexes = indexes;
        self
    }

    pub fn send(&mut self) -> Result<RefreshResult, EsError> {
        let url = format!("/{}/_refresh",
                          format_multi(&self.indexes));
        let response = try!(self.client.post_op(&url));
        match response.status_code() {
            &StatusCode::Ok => Ok(try!(response.read_response())),
            _              => Err(EsError::EsError(format!("Unexpected status: {}", response.status_code())))
        }
    }
}

impl Client {
    /// Refresh
    ///
    /// See: https://www.elastic.co/guide/en/elasticsearch/reference/1.x/indices-refresh.html
    pub fn refresh<'a>(&'a mut self) -> RefreshOperation {
        RefreshOperation::new(self)
    }
}

/// Result of a refresh request
#[derive(Deserialize)]
pub struct RefreshResult {
    #[serde(rename="_shards")]
    pub shards: ShardCountResult
}
