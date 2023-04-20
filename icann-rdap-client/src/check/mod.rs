use std::any::TypeId;

use icann_rdap_common::response::RdapResponse;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumMessage};

use crate::md::MdParams;

pub mod autnum;
pub mod domain;
pub mod entity;
pub mod error;
pub mod help;
pub mod nameserver;
pub mod network;
pub mod search;
pub mod types;

/// Describes the calls of checks.
#[derive(Debug, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckClass {
    #[strum(serialize = "Info")]
    Informational,
    #[strum(serialize = "SWrn")]
    SpecificationWarning,
    #[strum(serialize = "SErr")]
    SpecificationError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checks<'a> {
    pub struct_name: &'a str,
    pub items: Vec<CheckItem>,
    pub sub_checks: Vec<Checks<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckItem {
    pub check_class: CheckClass,
    pub check: Check,
}

pub trait GetChecks {
    fn get_checks(&self, params: CheckParams) -> Checks;
}

#[derive(Clone, Copy)]
pub struct CheckParams<'a> {
    pub do_subchecks: bool,
    pub root: &'a RdapResponse,
    pub parent_type: Option<TypeId>,
}

impl<'a> CheckParams<'a> {
    pub fn from_parent(&self, parent_type: TypeId) -> Self {
        CheckParams {
            do_subchecks: self.do_subchecks,
            root: self.root,
            parent_type: Some(parent_type),
        }
    }

    pub fn from_md(md_params: MdParams<'a>, parent_type: TypeId) -> Self {
        CheckParams {
            do_subchecks: false,
            root: md_params.root,
            parent_type: Some(parent_type),
        }
    }

    pub fn from_md_no_parent(md_params: MdParams<'a>) -> Self {
        CheckParams {
            do_subchecks: false,
            root: md_params.root,
            parent_type: md_params.parent_type,
        }
    }
}

impl GetChecks for RdapResponse {
    fn get_checks(&self, params: CheckParams) -> Checks {
        match &self {
            RdapResponse::Entity(e) => e.get_checks(params),
            RdapResponse::Domain(d) => d.get_checks(params),
            RdapResponse::Nameserver(n) => n.get_checks(params),
            RdapResponse::Autnum(a) => a.get_checks(params),
            RdapResponse::Network(n) => n.get_checks(params),
            RdapResponse::DomainSearchResults(r) => r.get_checks(params),
            RdapResponse::EntitySearchResults(r) => r.get_checks(params),
            RdapResponse::NameserverSearchResults(r) => r.get_checks(params),
            RdapResponse::ErrorResponse(e) => e.get_checks(params),
            RdapResponse::Help(h) => h.get_checks(params),
        }
    }
}

#[derive(Debug, EnumMessage, Serialize, Deserialize)]
pub enum Check {
    // RDAP Conformance
    #[strum(message = "'rdapConformance' can only appear at the top of response.")]
    InvalidRdapConformanceParent,

    // Links
    #[strum(message = "'value' property not found in Link structure as required by RFC 7083")]
    LinkMissingValueProperty,
    #[strum(message = "'rel' property not found in Link structure as required by RFC 7083")]
    LinkMissingRelProperty,
}
