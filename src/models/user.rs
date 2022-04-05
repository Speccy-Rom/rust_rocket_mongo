use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub description: String,
    pub is_admin: bool,
    pub acl_allow_ips: Vec<String>,
    pub acl_allow_endpoints: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) created_ip: String,
    pub(crate) created_by: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) email: String,
    pub(crate) description: String,
    pub(crate) api_key: String,
    pub(crate) is_admin: bool,
    pub(crate) acl_allow_ips: Vec<String>,
    pub(crate) acl_allow_endpoints: Vec<String>,
}

impl User {
    /// Check if request ip address is allowed to access from.
    ///
    /// Returns true if accessible ip is one that is stored in user's "acl_allow_ips".
    pub fn is_ip_accessible(&self, origin_ip: &str) -> bool {
        self.acl_allow_ips
            .iter()
            .any(|ip| ip.eq(origin_ip) || ip.eq("*"))
    }

    /// Check if endpoint is allowed to access.
    ///
    /// Returns true if endpoint is in acl_allow_endpoints
    pub fn is_endpoint_allowed(&self, origin_endpoint: &str) -> bool {
        self.acl_allow_endpoints
            .iter()
            .any(|endpoint| origin_endpoint.eq(endpoint) || endpoint.eq("*"))
    }

    /// Check if the api key is admin level api key
    ///
    /// Returns true if admin else false
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}
