#![allow(clippy::missing_safety_doc)]
/// Copyright 2023 Alexander Kalashnikov

///    Licensed under the Apache License, Version 2.0 (the "License");
///    you may not use this file except in compliance with the License.
///    You may obtain a copy of the License at

///        http://www.apache.org/licenses/LICENSE-2.0

///    Unless required by applicable law or agreed to in writing, software
///    distributed under the License is distributed on an "AS IS" BASIS,
///    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///    See the License for the specific language governing permissions and
///    limitations under the License.
pub mod uni;

pub unsafe fn inline_mpf_codec_capabilities_add(
    capabilities: *mut uni::mpf_codec_capabilities_t,
    sample_rates: std::os::raw::c_int,
    codec_name: *const i8,
) -> uni::apt_bool_t {
    let attribs = uni::apr_array_push((*capabilities).attrib_arr) as *mut uni::mpf_codec_attribs_t;
    inline_apt_string_assign(
        &mut (*attribs).name as _,
        codec_name,
        (*(*capabilities).attrib_arr).pool,
    );
    (*attribs).sample_rates = sample_rates;
    (*attribs).bits_per_sample = 0;
    (*attribs).frame_duration = uni::CODEC_FRAME_TIME_BASE as _; // In version 1.8.0 was introduced 'frame_duration' codec property. 10 ms was hardcoded in earlier versions
    uni::TRUE
}

pub unsafe fn inline_apt_string_assign(
    str: *mut uni::apt_str_t,
    src: *const i8,
    pool: *mut uni::apr_pool_t,
) {
    (*str).buf = std::ptr::null_mut() as _;
    (*str).length = if src.is_null() { 0 } else { libc::strlen(src) };
    if (*str).length > 0 {
        (*str).buf = uni::apr_pstrmemdup(pool, src, (*str).length);
    }
}

pub unsafe fn inline_mrcp_header_allocate(
    accessor: *mut uni::mrcp_header_accessor_t,
    pool: *mut uni::apr_pool_t,
) -> *mut libc::c_void {
    if !(*accessor).data.is_null() {
        return (*accessor).data;
    }
    if (*accessor).vtable.is_null() || (*(*accessor).vtable).allocate.is_none() {
        return std::ptr::null_mut() as _;
    }
    (*(*accessor).vtable).allocate.unwrap()(accessor, pool)
}

pub unsafe fn inline_apt_string_set(str: *mut uni::apt_str_t, src: *const i8) {
    (*str).buf = src as _;
    (*str).length = if src.is_null() { 0 } else { libc::strlen(src) }
}

pub unsafe fn inline_mrcp_generic_header_prepare(
    message: *mut uni::mrcp_message_t,
) -> *mut uni::mrcp_generic_header_t {
    inline_mrcp_header_allocate(
        &mut (*message).header.generic_header_accessor as _,
        (*message).pool,
    ) as _
}

pub unsafe fn inline_mrcp_resource_header_prepare(
    mrcp_message: *mut uni::mrcp_message_t,
) -> *mut libc::c_void {
    inline_mrcp_header_allocate(
        &mut (*mrcp_message).header.resource_header_accessor as _,
        (*mrcp_message).pool,
    )
}

pub unsafe fn inline_mrcp_resource_header_get(
    message: *const uni::mrcp_message_t,
) -> *mut libc::c_void {
    (*message).header.resource_header_accessor.data
}

pub unsafe fn inline_mrcp_resource_header_property_check(
    message: *const uni::mrcp_message_t,
    id: uni::apr_size_t,
) -> uni::apt_bool_t {
    inline_apt_header_section_field_check(
        &(*message).header.header_section as _,
        id + uni::GENERIC_HEADER_COUNT as usize,
    )
}

pub unsafe fn inline_apt_header_section_field_check(
    header: *const uni::apt_header_section_t,
    id: uni::apr_size_t,
) -> uni::apt_bool_t {
    let arr_size = (*header).arr_size;
    let arr = std::slice::from_raw_parts((*header).arr, arr_size);
    if id < arr_size {
        return if arr[id].is_null() {
            uni::FALSE
        } else {
            uni::TRUE
        };
    }
    uni::FALSE
}
