use fastly::geo::{ConnSpeed, ConnType, Continent, ProxyDescription, ProxyType};

use askama::Template;
use serde::{Deserialize, Serialize};

use std::net::IpAddr;

#[derive(Template)]
#[template(path = "geo-ip@edge.html")]
pub struct GeoIPTemplate<'a> {
    pub geo_ip_response: &'a GeoIPResponse,
}

#[derive(Serialize, Deserialize)]
pub struct GeoIPResponse {
    pub ip: IpAddr,
    pub isp: GeoIPISPInfo,
    pub geographic: GeoIPGeographicInfo,
    pub connection: GeoIPConnectionInfo,
    pub proxy: GeoIPProxyInfo,
}

#[derive(Serialize, Deserialize)]
pub struct GeoIPISPInfo {
    pub isp_name: String,
    pub isp_asn: u32,
}

impl Default for GeoIPISPInfo {
    fn default() -> Self {
        GeoIPISPInfo {
            isp_name: "No GeoIP Info Available".to_string(),
            isp_asn: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeoIPGeographicInfo {
    pub city: String,
    pub country: String,
    pub continent: Continent,
    pub postal_code: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for GeoIPGeographicInfo {
    fn default() -> Self {
        GeoIPGeographicInfo {
            city: "No GeoIP Info Available".to_string(),
            country: "No GeoIP Info Available".to_string(),
            continent: Continent::Other("Unknown".to_string()),
            postal_code: "No GeoIP Info Available".to_string(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeoIPProxyInfo {
    pub proxy_type: ProxyType,
    pub proxy_description: ProxyDescription,
}

impl Default for GeoIPProxyInfo {
    fn default() -> Self {
        GeoIPProxyInfo {
            proxy_type: ProxyType::Unknown,
            proxy_description: ProxyDescription::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeoIPConnectionInfo {
    pub connection_speed: ConnSpeed,
    pub connection_type: ConnType,
}

impl Default for GeoIPConnectionInfo {
    fn default() -> Self {
        GeoIPConnectionInfo {
            connection_speed: ConnSpeed::Other("Unknown".to_string()),
            connection_type: ConnType::Other("Unknown".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeoIPUserAgentInfo {
    pub user_agent: String,
}

impl Default for GeoIPUserAgentInfo {
    fn default() -> Self {
        GeoIPUserAgentInfo {
            user_agent: String::from("Agent Not known."),
        }
    }
}
