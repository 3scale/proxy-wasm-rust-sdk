// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::hostcalls;
use crate::types::*;
use std::time::{Duration, SystemTime};

pub trait Context {
    fn get_current_time(&self) -> SystemTime {
        hostcalls::get_current_time().unwrap()
    }

    fn get_property(&self, path: Vec<&str>) -> Option<Bytes> {
        hostcalls::get_property(path).unwrap()
    }

    fn set_property(&self, path: Vec<&str>, value: Option<&[u8]>) {
        hostcalls::set_property(path, value).unwrap()
    }

    fn get_shared_data(&self, key: &str) -> (Option<Bytes>, Option<u32>) {
        hostcalls::get_shared_data(key).unwrap()
    }

    fn set_shared_data(
        &self,
        key: &str,
        value: Option<&[u8]>,
        cas: Option<u32>,
    ) -> Result<(), Status> {
        hostcalls::set_shared_data(key, value, cas)
    }

    fn register_shared_queue(&self, name: &str) -> u32 {
        hostcalls::register_shared_queue(name).unwrap()
    }

    fn resolve_shared_queue(&self, vm_id: &str, name: &str) -> Option<u32> {
        hostcalls::resolve_shared_queue(vm_id, name).unwrap()
    }

    fn dequeue_shared_queue(&self, queue_id: u32) -> Result<Option<Bytes>, Status> {
        hostcalls::dequeue_shared_queue(queue_id)
    }

    fn enqueue_shared_queue(&self, queue_id: u32, value: Option<&[u8]>) -> Result<(), Status> {
        hostcalls::enqueue_shared_queue(queue_id, value)
    }

    fn dispatch_http_call(
        &self,
        upstream: &str,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
        trailers: Vec<(&str, &str)>,
        timeout: Duration,
    ) -> Result<u32, Status> {
        hostcalls::dispatch_http_call(upstream, headers, body, trailers, timeout)
    }

    fn on_http_call_response(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
    ) {
    }

    fn get_http_call_response_headers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpCallResponseHeaders).unwrap()
    }

    fn get_http_call_response_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::HttpCallResponseBody, start, max_size).unwrap()
    }

    fn get_http_call_response_trailers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpCallResponseTrailers).unwrap()
    }

    fn on_done(&mut self) -> bool {
        true
    }

    fn done(&self) {
        hostcalls::done().unwrap()
    }
}

pub trait RootContext: Context {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        true
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        true
    }

    fn get_configuration(&self) -> Option<Bytes> {
        hostcalls::get_configuration().unwrap()
    }

    fn set_tick_period(&self, period: Duration) {
        hostcalls::set_tick_period(period).unwrap()
    }

    fn on_tick(&mut self) {}

    fn on_queue_ready(&mut self, _queue_id: u32) {}

    fn on_log(&mut self) {}
}

pub trait StreamContext: Context {
    fn on_new_connection(&mut self) -> FilterStatus {
        FilterStatus::Continue
    }

    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> FilterStatus {
        FilterStatus::Continue
    }

    fn get_downstream_data(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::DownstreamData, start, max_size).unwrap()
    }

    fn set_downstream_data(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::DownstreamData, start, size, value).unwrap()
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) {}

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> FilterStatus {
        FilterStatus::Continue
    }

    fn get_upstream_data(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::UpstreamData, start, max_size).unwrap()
    }

    fn set_upstream_data(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::UpstreamData, start, size, value).unwrap()
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) {}

    fn on_log(&mut self) {}
}

pub trait HttpContext: Context {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> FilterHeadersStatus {
        FilterHeadersStatus::Continue
    }

    fn get_http_request_headers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpRequestHeaders).unwrap()
    }

    fn set_http_request_headers(&self, headers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpRequestHeaders, headers).unwrap()
    }

    fn get_http_request_header(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpRequestHeaders, &name).unwrap()
    }

    fn set_http_request_header(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpRequestHeaders, &name, value).unwrap()
    }

    fn add_http_request_header(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpRequestHeaders, &name, value).unwrap()
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> FilterDataStatus {
        FilterDataStatus::Continue
    }

    fn get_http_request_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::HttpRequestBody, start, max_size).unwrap()
    }

    fn set_http_request_body(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::HttpRequestBody, start, size, value).unwrap()
    }

    fn on_http_request_trailers(&mut self, _num_trailers: usize) -> FilterTrailersStatus {
        FilterTrailersStatus::Continue
    }

    fn get_http_request_trailers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpRequestTrailers).unwrap()
    }

    fn set_http_request_trailers(&self, trailers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpRequestTrailers, trailers).unwrap()
    }

    fn get_http_request_trailer(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpRequestTrailers, &name).unwrap()
    }

    fn set_http_request_trailer(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpRequestTrailers, &name, value).unwrap()
    }

    fn add_http_request_trailer(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpRequestTrailers, &name, value).unwrap()
    }

    fn resume_http_request(&self) {
        hostcalls::resume_http_request().unwrap()
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> FilterHeadersStatus {
        FilterHeadersStatus::Continue
    }

    fn get_http_response_headers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpResponseHeaders).unwrap()
    }

    fn set_http_response_headers(&self, headers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpResponseHeaders, headers).unwrap()
    }

    fn get_http_response_header(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpResponseHeaders, &name).unwrap()
    }

    fn set_http_response_header(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpResponseHeaders, &name, value).unwrap()
    }

    fn add_http_response_header(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpResponseHeaders, &name, value).unwrap()
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> FilterDataStatus {
        FilterDataStatus::Continue
    }

    fn get_http_response_body(&self, start: usize, max_size: usize) -> Option<Bytes> {
        hostcalls::get_buffer(BufferType::HttpResponseBody, start, max_size).unwrap()
    }

    fn set_http_response_body(&self, start: usize, size: usize, value: &[u8]) {
        hostcalls::set_buffer(BufferType::HttpResponseBody, start, size, value).unwrap()
    }

    fn on_http_response_trailers(&mut self, _num_trailers: usize) -> FilterTrailersStatus {
        FilterTrailersStatus::Continue
    }

    fn get_http_response_trailers(&self) -> Vec<(String, String)> {
        hostcalls::get_map(MapType::HttpResponseTrailers).unwrap()
    }

    fn set_http_response_trailers(&self, headers: Vec<(&str, &str)>) {
        hostcalls::set_map(MapType::HttpResponseTrailers, headers).unwrap()
    }

    fn get_http_response_trailer(&self, name: &str) -> Option<String> {
        hostcalls::get_map_value(MapType::HttpResponseTrailers, &name).unwrap()
    }

    fn set_http_response_trailer(&self, name: &str, value: Option<&str>) {
        hostcalls::set_map_value(MapType::HttpResponseTrailers, &name, value).unwrap()
    }

    fn add_http_response_trailer(&self, name: &str, value: &str) {
        hostcalls::add_map_value(MapType::HttpResponseTrailers, &name, value).unwrap()
    }

    fn resume_http_response(&self) {
        hostcalls::resume_http_response().unwrap()
    }

    fn send_http_response(
        &self,
        status_code: u32,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
    ) {
        hostcalls::send_http_response(status_code, headers, body).unwrap()
    }

    fn clear_http_route_cache(&self) {
        hostcalls::clear_http_route_cache().unwrap()
    }

    fn on_log(&mut self) {}
}
