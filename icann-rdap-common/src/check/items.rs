use super::{Check, CheckClass, CheckItem};

impl CheckItem {
    // RDAP Conformance

    pub fn invalid_rdap_conformance_parent() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::InvalidRdapConformanceParent,
        }
    }

    // Links

    pub fn link_missing_value_property() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::LinkMissingValueProperty,
        }
    }
    pub fn related_link_is_not_rdap() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::RelatedLinkIsNotRdap,
        }
    }
    pub fn related_link_has_no_type() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::RelatedLinkHasNoType,
        }
    }
    pub fn self_link_is_not_rdap() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::SelfLinkIsNotRdap,
        }
    }
    pub fn self_link_has_no_type() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::SelfLinkHasNoType,
        }
    }
    pub fn object_class_has_no_self_link() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::ObjectClassHasNoSelfLink,
        }
    }
    pub fn link_missing_rel_property() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::LinkMissingRelProperty,
        }
    }

    // Variants

    pub fn empty_domain_variant() -> CheckItem {
        CheckItem {
            check_class: super::CheckClass::SpecificationWarning,
            check: Check::EmptyDomainVariant,
        }
    }

    // Events
    pub fn event_date_is_not_rfc3339() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::EventDateIsNotRfc3339,
        }
    }

    // Handle
    pub fn handle_is_empty() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::HandleIsEmpty,
        }
    }

    // Status
    pub fn status_is_empty() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::StatusIsEmpty,
        }
    }

    // Roles
    pub fn roles_are_empty() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::RolesAreEmpty,
        }
    }

    // LDH Name
    pub fn invalid_ldh_name() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::InvalidLdhName,
        }
    }

    // Network or Autnum Name
    pub fn name_is_empty() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::NameIsEmpty,
        }
    }

    // Network or Autnum Type
    pub fn type_is_empty() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::TypeIsEmpty,
        }
    }

    // IP Address
    pub fn missing_ip_address() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::MissingIpAddress,
        }
    }
    pub fn malformed_ip_address() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::MalformedIpAddress,
        }
    }
    pub fn end_ip_before_start_ip() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::EndIpBeforeStartIp,
        }
    }
    pub fn ip_version_mismatch() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationWarning,
            check: Check::IpVersionMismatch,
        }
    }
    pub fn malfomred_ip_version() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::MalformedIPVersion,
        }
    }
    pub fn ip_address_list_is_empty() -> CheckItem {
        CheckItem {
            check_class: CheckClass::SpecificationError,
            check: Check::IpAddressListIsEmpty,
        }
    }
}
