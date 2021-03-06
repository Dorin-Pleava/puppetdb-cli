use std::io::Write;
use url::Url;

use multipart::client::Multipart;
use hyper::header::{Connection, UserAgent};
use hyper::method::Method;
use kitchensink::net::Auth;
use kitchensink::utils::HyperResult;

use super::client::PdbClient;

/// POSTs a multipart request to PuppetDB for importing an archive.
pub fn post_import(pdb_client: &PdbClient, path: String) -> HyperResult {
    let server_url: String = pdb_client.server_urls[0].clone();
    let url = Url::parse(&(server_url + "/pdb/admin/v1/archive")).unwrap();
    let request = Auth::request(&pdb_client.auth, Method::Post, url);
    let mut multipart = Multipart::from_request(request).unwrap();
    multipart.write_file("archive", &path)
        .unwrap_or_else(|e| pretty_panic!("Error writing archive to request: {}", e));
    multipart.send()
}

pub fn get_export(pdb_client: &PdbClient, anonymization: String) -> HyperResult {
    let server_url: String = pdb_client.server_urls[0].clone();
    let query_params = "?anonymization_profile=".to_string() + &anonymization;
    let cli = Auth::client(&pdb_client.auth);

    let req = cli.get(&(server_url + "/pdb/admin/v1/archive" + &query_params))
        .header(UserAgent("puppetdb-cli".to_owned()))
        .header(Connection::close());
    Auth::auth_header(&pdb_client.auth, req).send()
}
