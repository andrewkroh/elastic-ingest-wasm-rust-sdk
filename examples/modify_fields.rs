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

#[no_mangle]
pub extern "C" fn process() -> Status {
    put_field("string", r#""hello""#).unwrap();
    put_field("integer", "1").unwrap();
    put_field("float", "1.2").unwrap();
    put_field("bool", "true").unwrap();
    put_field("object", r#"{"hello":"world!"}"#).unwrap();
    put_field("null", "null").unwrap();
    return Status::Ok;
}
