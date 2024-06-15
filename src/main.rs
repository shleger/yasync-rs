extern crate google_photoslibrary1 as photoslibrary1;
use crate::oauth2::ApplicationSecret;
use photoslibrary1::{hyper, hyper_rustls, oauth2, PhotosLibrary};
use serde_json::Value;
use std::fs;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.

    let data = fs::read_to_string("client_secret.json").unwrap(); //
    let v: Value = serde_json::from_str(data.as_str()).unwrap();
    let s = v["installed"].to_string();
    let secret: ApplicationSecret = serde_json::from_str(s.as_str()).unwrap();

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    let client = hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build(),
    );

    let hub = PhotosLibrary::new(client, auth);
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !

    let media_items = hub.media_items().list().page_size(2).doit().await.unwrap();
    let media_item = &media_items.1.media_items.as_ref().unwrap()[0];
    let image_bytes = reqwest::get(media_item.base_url.as_ref().unwrap().to_owned() + "=d")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    File::create(media_item.filename.as_ref().unwrap())
        .await
        .unwrap()
        .write_all(&*image_bytes)
        .await
        .unwrap();
}

#[test]
fn test_read_secret() {
    use crate::oauth2::ApplicationSecret;
    use serde_json::Value;
    use std::fs;

    let data = fs::read_to_string("test_secret.json").unwrap(); //
    let v: Value = serde_json::from_str(data.as_str()).unwrap();
    let s = v["installed"].to_string();
    let a: ApplicationSecret = serde_json::from_str(s.as_str()).unwrap();
    assert_eq!(a.client_id, "1");
    assert_eq!(a.project_id.unwrap(), "2");
    assert_eq!(a.auth_uri, "3");
    assert_eq!(a.token_uri, "4");
    assert_eq!(a.auth_provider_x509_cert_url.unwrap(), "5");
    assert_eq!(a.client_secret, "6");
    assert_eq!(a.redirect_uris[0], "7");
}
