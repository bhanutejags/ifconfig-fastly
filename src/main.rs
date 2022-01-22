// use fastly::dictionary::Dictionary;
use fastly::error::anyhow;
use fastly::geo::geo_lookup;
use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

use askama::Template;

use ifconfig_fastly::*;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // let config_dictionary = Dictionary::open("ifconfig_fastly_config");
    // let logging_endpoint_name = config_dictionary
    //     .get("LOGGING_ENDPOINT_NAME")
    //     .unwrap_or_else(|| String::from("AWS S3 Logging Endpoint"));

    log_fastly::init_simple("AWS S3 Logging Endpoint", log::LevelFilter::Info);
    fastly::log::set_panic_endpoint("AWS S3 Logging Endpoint")?;
    log::info!("Received a request.");

    match req.get_method() {
        &Method::GET | &Method::HEAD => (),

        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    let client_ip = req
        .get_client_ip_addr()
        .ok_or_else(|| anyhow!("Could not get Client IP"))?;

    log::info!("Received a request from IP: {}.", client_ip);

    let response = match geo_lookup(client_ip) {
        Some(geo) => {
            log::debug!("Got Geographic information.");
            GeoIPResponse {
                ip: client_ip,
                isp: GeoIPISPInfo {
                    isp_name: geo.as_name().to_string(),
                    isp_asn: geo.as_number(),
                },
                geographic: GeoIPGeographicInfo {
                    city: geo.city().to_string(),
                    country: geo.country_name().to_string(),
                    continent: geo.continent(),
                    postal_code: geo.postal_code().to_string(),
                    latitude: geo.latitude(),
                    longitude: geo.longitude(),
                },
                connection: GeoIPConnectionInfo {
                    connection_type: geo.conn_type(),
                    connection_speed: geo.conn_speed(),
                },
                proxy: GeoIPProxyInfo {
                    proxy_type: geo.proxy_type(),
                    proxy_description: geo.proxy_description(),
                },
            }
        }
        None => {
            log::error!(
                "Could not get Geographic information from IP {}.",
                client_ip
            );
            GeoIPResponse {
                ip: client_ip,
                isp: GeoIPISPInfo::default(),
                geographic: GeoIPGeographicInfo::default(),
                connection: GeoIPConnectionInfo::default(),
                proxy: GeoIPProxyInfo::default(),
            }
        }
    };

    match req.get_path() {
        "/" => {
            let geo_ip_response_template = GeoIPTemplate {
                geo_ip_response: &response,
            };
            let geo_ip_response_template_rendered = geo_ip_response_template.render()?;

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(geo_ip_response_template_rendered))
        }
        "/json" => Ok(Response::from_status(StatusCode::OK).with_body_json(&response)?),

        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
