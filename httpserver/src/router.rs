use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler,
    WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            //if GET Request
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    //Parse the URI
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        //if route begins with /api -> invoce Web service
                        "api" => {
                            let resp: HttpResponse = 
                            WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        //else invoce static page handler
                        _ => {
                            let resp: HttpResponse = 
                            StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            //if Request is not a GET Request -> return 404 page
            _ => {
                let resp: HttpResponse = 
                PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}