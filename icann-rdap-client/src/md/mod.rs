use std::{any::TypeId, char};

use icann_rdap_common::response::RdapResponse;

use crate::{check::CheckClass, request::RequestData};

pub mod autnum;
pub mod domain;
pub mod entity;
pub mod error;
pub mod help;
pub mod nameserver;
pub mod network;
pub mod search;
pub mod types;

pub(crate) const _CODE_INDENT: &str = "    ";

pub(crate) const HR: &str = "----------------------------------------\n";

/// Specifies options for generating markdown.
pub struct MdOptions {
    /// If true, do not use Unicode characters.
    pub no_unicode_chars: bool,

    /// The character used for text styling of bold and italics.
    pub text_style_char: char,

    /// If true, headers use the hash marks or under lines.
    pub hash_headers: bool,

    /// If true, the text_style_char will appear in a justified text.
    pub style_in_justify: bool,
}

impl Default for MdOptions {
    fn default() -> Self {
        MdOptions {
            no_unicode_chars: false,
            text_style_char: '*',
            hash_headers: true,
            style_in_justify: false,
        }
    }
}

impl MdOptions {
    /// Defaults for markdown that looks more like plain text.
    pub fn plain_text() -> Self {
        MdOptions {
            no_unicode_chars: true,
            text_style_char: '_',
            hash_headers: false,
            style_in_justify: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MdParams<'a> {
    pub heading_level: usize,
    pub root: &'a RdapResponse,
    pub parent_type: Option<TypeId>,
    pub check_types: &'a [CheckClass],
    pub options: &'a MdOptions,
    pub req_data: &'a RequestData<'a>,
}

impl<'a> MdParams<'a> {
    pub fn from_parent(&self, parent_type: TypeId) -> Self {
        MdParams {
            parent_type: Some(parent_type),
            heading_level: self.heading_level,
            root: self.root,
            check_types: self.check_types,
            options: self.options,
            req_data: self.req_data,
        }
    }
}

pub trait ToMd {
    fn to_md(&self, params: MdParams) -> String;
}

impl ToMd for RdapResponse {
    fn to_md(&self, params: MdParams) -> String {
        let mut md = String::new();
        let variant_md = match &self {
            RdapResponse::Entity(entity) => entity.to_md(params),
            RdapResponse::Domain(domain) => domain.to_md(params),
            RdapResponse::Nameserver(nameserver) => nameserver.to_md(params),
            RdapResponse::Autnum(autnum) => autnum.to_md(params),
            RdapResponse::Network(network) => network.to_md(params),
            RdapResponse::DomainSearchResults(results) => results.to_md(params),
            RdapResponse::EntitySearchResults(results) => results.to_md(params),
            RdapResponse::NameserverSearchResults(results) => results.to_md(params),
            RdapResponse::ErrorResponse(error) => error.to_md(params),
            RdapResponse::Help(help) => help.to_md(params),
        };
        md.push_str(&variant_md);
        md.push_str(HR);
        md
    }
}

pub(crate) fn to_em(str: &str, options: &MdOptions) -> String {
    format!(
        "{}{str}{}",
        options.text_style_char, options.text_style_char
    )
}

pub(crate) fn to_bold(str: &str, options: &MdOptions) -> String {
    format!(
        "{}{}{str}{}{}",
        options.text_style_char,
        options.text_style_char,
        options.text_style_char,
        options.text_style_char
    )
}

pub(crate) fn to_header(str: &str, level: usize, options: &MdOptions) -> String {
    if options.hash_headers {
        format!("{} {str}\n\n", "#".repeat(level))
    } else {
        let line = if level == 1 {
            "=".repeat(str.len())
        } else {
            "-".repeat(str.len())
        };
        format!("{str}\n{line}\n\n")
    }
}

pub(crate) fn to_right(str: &str, width: usize, options: &MdOptions) -> String {
    if options.no_unicode_chars {
        format!("{str:>width$}")
    } else {
        format!("{str:\u{2003}>width$}")
    }
}

pub(crate) fn to_right_em(str: &str, width: usize, options: &MdOptions) -> String {
    if options.style_in_justify {
        to_right(&to_em(str, options), width, options)
    } else {
        to_em(&to_right(str, width, options), options)
    }
}

#[allow(dead_code)]
pub(crate) fn to_right_bold(str: &str, width: usize, options: &MdOptions) -> String {
    if options.style_in_justify {
        to_right(&to_bold(str, options), width, options)
    } else {
        to_bold(&to_right(str, width, options), options)
    }
}

pub(crate) fn to_left(str: &str, width: usize, options: &MdOptions) -> String {
    if options.no_unicode_chars {
        format!("{str:<width$}")
    } else {
        format!("{str:\u{2003}<width$}")
    }
}

#[allow(dead_code)]
pub(crate) fn to_left_em(str: &str, width: usize, options: &MdOptions) -> String {
    if options.style_in_justify {
        to_left(&to_em(str, options), width, options)
    } else {
        to_em(&to_left(str, width, options), options)
    }
}

#[allow(dead_code)]
pub(crate) fn to_left_bold(str: &str, width: usize, options: &MdOptions) -> String {
    if options.style_in_justify {
        to_left(&to_bold(str, options), width, options)
    } else {
        to_bold(&to_left(str, width, options), options)
    }
}

#[allow(dead_code)]
pub(crate) fn to_center(str: &str, width: usize, options: &MdOptions) -> String {
    if options.no_unicode_chars {
        format!("{str:^width$}")
    } else {
        format!("{str:\u{2003}^width$}")
    }
}

#[allow(dead_code)]
pub(crate) fn to_center_em(str: &str, width: usize, options: &MdOptions) -> String {
    if options.style_in_justify {
        to_center(&to_em(str, options), width, options)
    } else {
        to_em(&to_center(str, width, options), options)
    }
}

#[allow(dead_code)]
pub(crate) fn to_center_bold(str: &str, width: usize, options: &MdOptions) -> String {
    if options.style_in_justify {
        to_center(&to_bold(str, options), width, options)
    } else {
        to_bold(&to_center(str, width, options), options)
    }
}