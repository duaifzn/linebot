use crate::dto::line_dto::WebhookDto;
use crate::util::line_signature::create_signature;
use crate::config::Config;
use rocket::data::{self, Data, FromData, Outcome};
use rocket::http::Status;
use rocket::request::Request;
use rocket::serde::json::serde_json;

lazy_static!(
    static ref CONFIG: Config<'static> = Config::load();
);

#[derive(Debug)]
pub enum BodyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromData<'r> for WebhookDto {
    type Error = BodyError;

    async fn from_data(req: &'r Request<'_>, mut data: Data<'r>) -> data::Outcome<'r, Self> {
        let result = std::str::from_utf8(data.peek(512).await);
        if result.is_err() {
            return Outcome::Failure((Status::ExpectationFailed, BodyError::Invalid))
        }
        let body = result.unwrap();
        let sign1 = create_signature(CONFIG.secret.to_string(), body.to_string());
        match req.headers().get_one("x-line-signature") {
            Some(sign) => {
                if sign.to_string() != sign1{
                    return Outcome::Failure((Status::Unauthorized, BodyError::Invalid))
                }
                let webhook_data: WebhookDto = serde_json::from_str(body).unwrap();
                Outcome::Success(webhook_data)
            }
            None => Outcome::Failure((Status::NonAuthoritativeInformation, BodyError::Missing)),
        }
    }
}
