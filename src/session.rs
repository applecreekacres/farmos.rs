use std::collections::HashMap;

extern crate reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use reqwest::RedirectPolicy;
use reqwest::Response;
use reqwest::StatusCode;

use crate::logging;

pub enum Method {
    Post,
    Put,
    Get,
}

pub struct Session {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub token: String,
    pub authenticated: bool,
    client: reqwest::Client,
}

impl Session {
    pub fn init(host: String, username: String, password: String) -> Self {
        let end: &[char] = &['/'];
        let trimmed_host = host.trim_end_matches(end).to_string();
        logging::logging_init();
        Session {
            hostname: trimmed_host,
            username: username,
            password: password,
            token: String::new(),
            authenticated: false,
            client: Client::builder()
                .cookie_store(true)
                .redirect(RedirectPolicy::none())
                .build()
                .unwrap(),
        }
    }

    pub fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.token = String::new();

        let data = [
            ("name", self.username.as_str()),
            ("pass", self.password.as_str()),
            ("form_id", "user_login"),
        ];

        // let resp = self.request("user/login", Method::Post, HashMap::new(), data.clone(), true);
        // let resp2 = self.request("restws/session/token", Method::Get, HashMap::new(), data.clone(), true);
        let url = format!("{}/{}", self.hostname, "user/login");
        info!("Accessing URL {}", url);
        let mut resp = self.client.post(url.as_str()).form(&data).send()?;

        while resp.status() != StatusCode::from_u16(200)? {
            let location = resp.headers().get("Location").unwrap().to_str()?;
            resp = if resp.status() == StatusCode::from_u16(302).unwrap() {
                self.client.get(location).form(&data).send()?
            } else {
                self.client.post(location).form(&data).send()?
            };
            for cookie in resp.cookies() {
                println!("{} - {}", cookie.name(), cookie.value());
            }
        }

        let url2 = format!("{}/{}", self.hostname, "restws/session/token");
        let mut resp2 = self.client.get(url2.as_str()).send()?;
        while resp2.status() != StatusCode::from_u16(200)? {
            let location = resp2.headers().get("Location").unwrap().to_str()?;
            resp2 = if resp2.status() == StatusCode::from_u16(302).unwrap() {
                self.client.get(location).form(&data).send()?
            } else {
                self.client.post(location).form(&data).send()?
            };
        }
        self.token = resp2.text()?;
        debug!("{}", self.token);
        Ok(())
    }

    //     pub fn request(
    //         &self,
    //         path: &'static str,
    //         method: Method,
    //         options: HashMap<String, String>,
    //         data: HashMap<&str, &str>,
    //         force: bool,
    //     ) -> Result<Response, Box<dyn std::error::Error>> {
    //         let url = format!("{}/{}", self.hostname, path);
    //         let mut headers = HeaderMap::new();
    //         let token: String = match options.get("X-CSRF-Token") {
    //             Some(ref value) => value.to_string(),
    //             None => String::from(""),
    //         };

    //         if self.token != String::new() {
    //             headers.insert("X-CSRF-Token", HeaderValue::from_str(&self.token).unwrap());
    //         }

    //         // TODO Need to expand this to all method types, not just POST
    //         let mut resp = match method {
    //             Method::Post => self
    //                 .client
    //                 .post(url.as_str())
    //                 .headers(headers)
    //                 .form(&data)
    //                 .send()?,
    //             Method::Get => self
    //                 .client
    //                 .get(url.as_str())
    //                 // .headers(headers)
    //                 .form(&data)
    //                 .send()?,
    //             Method::Put => self
    //                 .client
    //                 .put(url.as_str())
    //                 .headers(headers)
    //                 .form(&data)
    //                 .send()?,
    //         };

    //         while resp.status() != StatusCode::from_u16(200)? {
    //             let location = resp.headers().get("Location").unwrap().to_str()?;
    //             resp = if resp.status() == StatusCode::from_u16(302).unwrap() {
    //                 self.client.get(location).form(&data).send()?
    //             } else {
    //                 self.client.post(location).form(&data).send()?
    //             };
    //         }

    //         Ok(resp)
    //     }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn login() {
        let mut sess = Session::init(
            "http://applecreekacres.farmos.net".to_string(),
            "lucasbrendel".to_string(),
            "uvtdLx3S".to_string(),
        );
        let res = sess.authenticate();
        // assert_eq!((), sess.authenticate())
    }
}
