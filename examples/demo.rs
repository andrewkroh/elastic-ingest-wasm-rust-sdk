// Licensed to Elasticsearch B.V. under one or more contributor
// license agreements. See the NOTICE file distributed with
// this work for additional information regarding copyright
// ownership. Elasticsearch B.V. licenses this file to you under
// the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use elastic_ingest::hostcalls::*;
use elastic_ingest::types::*;

use chrono::prelude::{DateTime, Utc};

#[no_mangle]
pub extern "C" fn process() -> Status {
    // Get the current time and write it to event.created.
    put_value_as_json(
        "event.created",
        serde_json::json!(iso8601(&get_current_time().unwrap())),
    )
    .unwrap();

    let event_original_optional = get_field("event.original").unwrap();
    if event_original_optional.is_none() {
        return Status::Ok;
    }

    let event_original_raw_json = event_original_optional.unwrap();

    // Log a message to the host.
    log(
        LogLevel::Debug,
        format!(
            "get_field returned event.original='{}'",
            event_original_raw_json.as_str()
        )
        .as_str(),
    );

    // Copy event.original to message.
    put_field("message", event_original_raw_json.as_str()).unwrap();

    return Status::Ok;
}

// iso8601 formats the time like "2001-07-08T00:34:60.026490+09:30".
fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

fn put_value_as_json(key: &str, val: serde_json::Value) -> Result<(), Status> {
    let json = serde_json::to_string(&val).unwrap();
    return put_field(key, json.as_str());
}
