extern crate google_photoslibrary1 as photoslibrary1;
use photoslibrary1::{hyper, hyper_rustls, oauth2, Error, PhotosLibrary};
use std::default::Default;



#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main()  {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret: oauth2::ApplicationSecret = Default::default();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
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
    let hub = PhotosLibrary::new(
        client,
        auth,
    );
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !

    let result = hub
        .albums()
        .list()
        .page_token("et")
        .page_size(-33)
        .exclude_non_app_created_data(true)
        .doit()
        .await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}
