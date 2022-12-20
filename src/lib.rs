use std::{collections::HashMap, io::Cursor};

use image::imageops::FilterType;
use image::io::Reader;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::debug;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ResizerActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ResizerActor {
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // TODO: Maybe support mime type guessing instead in the future

        debug!("Got body with size: {}", req.body.len());

        let mut parsed_image = match Reader::new(Cursor::new(&req.body))
            .with_guessed_format()
            .map(|r| r.decode())
        {
            Ok(Ok(img)) => img,
            Err(e) => {
                // According to the docs an error only happens on the guess format when there is an
                // IO error. This data is all in memory, so it shouldn't happen. Handling the error
                // regardless anyway
                return Ok(HttpResponse::bad_request(e));
            }
            Ok(Err(e)) => {
                return Ok(HttpResponse::bad_request(format!(
                    "Unable to decode image data from body: {:?}",
                    e
                )))
            }
        };

        let longest_side = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "longest_side_pixels")
            .map(|(_, v)| v.parse::<u32>().ok())
            .flatten()
            .filter(|size| size < &parsed_image.width().max(parsed_image.height()));

        if let Some(size) = longest_side {
            debug!("Got size of {}", size);
            // This maintains aspect ratio, so putting the same size should resize for the longest
            // size
            parsed_image = parsed_image.resize(size, size, FilterType::Lanczos3);
        }

        let mut converted: Vec<u8> = Vec::new();
        if let Err(e) = parsed_image.write_to(
            &mut Cursor::new(&mut converted),
            image::ImageOutputFormat::WebP,
        ) {
            return Ok(HttpResponse::internal_server_error(format!(
                "Unable to convert image to webp format: {:?}",
                e
            )));
        }

        let mut header = HashMap::new();
        header.insert("Content-Type".to_string(), vec!["image/webp".to_string()]);

        Ok(HttpResponse {
            body: converted,
            status_code: 200,
            header,
        })
    }
}
