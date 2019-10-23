use std::result::Result;

use reqwest::blocking::Response;
use reqwest::Error;
use serde::Deserialize;

use crate::api::APIBuilder;

#[derive(Deserialize, Debug)]
pub struct CmServerWebApi {
    response: CmServerResponse,
}

#[derive(Deserialize, Debug)]
pub struct CmServerResponse {
    serverlist: Vec<String>,
    serverlist_websockets: Vec<String>,
    result: u8,
    message: String,
}
impl CmServerWebApi {
    pub fn dump_servers(&self) -> Vec<String> {
        self.response.serverlist.clone()
    }
}

/// Requests login servers from Steam WEB API
/// Steam calls regions as Cells
/// reference: https://github.com/SteamDatabase/SteamTracking/blob/master/ClientExtracted/steam/cached/CellMap.vdf
pub fn fetch_servers(api_key: &str) -> Result<CmServerWebApi, Error> {
    let parameters = vec![("cellid", "0")];

    let new_api_call =
        APIBuilder::new("ISteamDirectory", "GetCMList", api_key, Option::from(parameters));

    let json: CmServerWebApi = reqwest::blocking::get(new_api_call.dump_request_link())?.json()?;
    Ok(json)
}

/// Requests server from cm0.steampowered.com
pub fn fetch_servers_fallback(api_key: &str) -> Result<String, Error> {
    let url = "cm0.steampowered.com";

    unimplemented!()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn fetch_servers_web_api() {
        let get_results = fetch_servers(env!("STEAM_API"));
        let servers: CmServerWebApi = get_results.unwrap();
        println!("{:?}", servers.response.serverlist);
    }
}
