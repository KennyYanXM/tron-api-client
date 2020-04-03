use reqwest::{header::HeaderValue, Client as HttpClient, Method, RequestBuilder, Response};
use url::Url;
use crate::response::{NodeInfo, Block, Error as ResponseError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use crate::params::*;
use crate::error::{Error, Result};
use serde_json;

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    // private_key: String,
    http_client: HttpClient,
}

/*
// cannot count on http status... errors have following JSON with 200 status:
// example:
// body = "{\"Error\":\"class java.lang.NumberFormatException : null\"}\n"
//
fn api_errors(res: &Response) -> Result<()> {
    match res.status().into() {
        404 => Err(Error::NotFound),
        500..=599 => Err(Error::ServerError),
        _ => Ok(()),
    }
}
*/

async fn decode_response<T>(res: Response) -> Result<T>
where
    T: DeserializeOwned
{
    let data = res.text().await?;

    let s: T = serde_json::from_str(&data).map_err(|_| {
        match serde_json::from_str(&data) {
            Err(err) => err.into(),
            Ok(r) => Error::ServerError(r)
        }
    })?;

    Ok(s)
}


impl Client {
    pub fn new(base_url: String) -> Self {
        Client {
            base_url: Url::parse(&base_url).expect("could not parse base_url"),
            http_client: HttpClient::new(),
        }
    }
    // todo: for_network(shasta) -> Client (uses trongrid.io api url for shasta

    pub async fn node_info(&self) -> Result<NodeInfo> {
        let res = self
            .prep_req(Method::GET, self.node_info_url())
            .await?
            .send()
            .await?;
        decode_response::<NodeInfo>(res).await
    }

    pub async fn get_block_by_num(&self, num: u32) -> Result<Block> {
        let res = self
            .prep_req(Method::POST, self.get_block_by_num_url())
            .await?
            .json(&GetBlockByNumParams::new(num))
            .send()
            .await?;
        decode_response::<Block>(res).await
    }

    /*
    pub async fn series_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }
    */


    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json");
            /*
            .bearer_auth(
                &self
                    .token
                    .lock()
                    .await
                    .as_ref()
                    .expect("missing token although ensured valid")
                    .token,
            );
            */

        Ok(req)
    }

    /*
    fn login_url(&self) -> Url {
        self.base_url
            .join("/login")
            .expect("could not parse login url")
    }
    */

    fn node_info_url(&self) -> Url {
        self.base_url
            .join("/wallet/getnodeinfo")
            .expect("could not parse nodeinfo")
    }

    fn get_block_by_num_url(&self) -> Url {
        self.base_url
            .join("/wallet/getblockbynum")
            .expect("could not parse nodeinfo")
    }

    /*
    fn series_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}", id))
            .expect("could not parse series url")
    }
    */
}

/*
#[cfg(test)]
mod tests;
*/
