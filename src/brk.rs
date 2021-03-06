/* Zed Attack Proxy (ZAP) and its related class files.
 *
 * ZAP is an HTTP/HTTPS proxy for assessing web application security.
 *
 * Copyright 2019 the ZAP development team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::ZapApiError;
use super::ZapService;
use serde_json::Value;
use std::collections::HashMap;

/**
 * This file was automatically generated.
 */
/**
 * Returns True if ZAP will break on both requests and responses
*/
pub fn is_break_all(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "isBreakAll", params)
}

/**
 * Returns True if ZAP will break on requests
*/
pub fn is_break_request(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "isBreakRequest", params)
}

/**
 * Returns True if ZAP will break on responses
*/
pub fn is_break_response(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "isBreakResponse", params)
}

/**
 * Returns the HTTP message currently intercepted (if any)
*/
pub fn http_message(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "view", "httpMessage", params)
}

/**
 * Controls the global break functionality. The type may be one of: http-all, http-request or http-response. The state may be true (for turning break on for the specified type) or false (for turning break off). Scope is not currently used.
*/
pub fn brk(
    service: &ZapService,
    typ: String,
    state: String,
    scope: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("type".to_string(), typ);
    params.insert("state".to_string(), state);
    params.insert("scope".to_string(), scope);
    super::call(service, "break", "action", "break", params)
}

/**
 * Overwrites the currently intercepted message with the data provided
*/
pub fn set_http_message(
    service: &ZapService,
    httpheader: String,
    httpbody: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("httpHeader".to_string(), httpheader);
    params.insert("httpBody".to_string(), httpbody);
    super::call(service, "break", "action", "setHttpMessage", params)
}

/**
 * Submits the currently intercepted message and unsets the global request/response break points
*/
pub fn cont(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "action", "continue", params)
}

/**
 * Submits the currently intercepted message, the next request or response will automatically be intercepted
*/
pub fn step(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "action", "step", params)
}

/**
 * Drops the currently intercepted message
*/
pub fn drop(service: &ZapService) -> Result<Value, ZapApiError> {
    let params = HashMap::new();
    super::call(service, "break", "action", "drop", params)
}

/**
 * Adds a custom HTTP breakpont. The string is the string to match. Location may be one of: url, request_header, request_body, response_header or response_body. Match may be: contains or regex. Inverse (match) may be true or false. Lastly, ignorecase (when matching the string) may be true or false.  
*/
pub fn add_http_breakpoint(
    service: &ZapService,
    string: String,
    location: String,
    mtch: String,
    inverse: String,
    ignorecase: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("string".to_string(), string);
    params.insert("location".to_string(), location);
    params.insert("match".to_string(), mtch);
    params.insert("inverse".to_string(), inverse);
    params.insert("ignorecase".to_string(), ignorecase);
    super::call(service, "break", "action", "addHttpBreakpoint", params)
}

/**
 * Removes the specified break point
*/
pub fn remove_http_breakpoint(
    service: &ZapService,
    string: String,
    location: String,
    mtch: String,
    inverse: String,
    ignorecase: String,
) -> Result<Value, ZapApiError> {
    let mut params = HashMap::new();
    params.insert("string".to_string(), string);
    params.insert("location".to_string(), location);
    params.insert("match".to_string(), mtch);
    params.insert("inverse".to_string(), inverse);
    params.insert("ignorecase".to_string(), ignorecase);
    super::call(service, "break", "action", "removeHttpBreakpoint", params)
}
