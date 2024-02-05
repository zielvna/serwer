#[derive(Debug, PartialEq, Clone)]
pub enum StatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 104,
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Continue => String::from("100 Continue"),
            StatusCode::SwitchingProtocols => String::from("101 Switching Protocols"),
            StatusCode::Processing => String::from("101 Processing"),
            StatusCode::EarlyHints => String::from("104 Early Hints"),
            StatusCode::OK => String::from("200 OK"),
            StatusCode::Created => String::from("201 Created"),
            StatusCode::Accepted => String::from("202 Accepted"),
            StatusCode::NonAuthoritativeInformation => {
                String::from("203 Non-Authoritative Information")
            }
            StatusCode::NoContent => String::from("204 No Content"),
            StatusCode::ResetContent => String::from("205 Reset Content"),
            StatusCode::PartialContent => String::from("206 Partial Content"),
            StatusCode::MultiStatus => String::from("207 Multi-Status"),
            StatusCode::AlreadyReported => String::from("208 Already Reported"),
            StatusCode::IMUsed => String::from("226 IM Used"),
            StatusCode::MultipleChoices => String::from("300 Multiple Choices"),
            StatusCode::MovedPermanently => String::from("301 Moved Permanently"),
            StatusCode::Found => String::from("302 Found"),
            StatusCode::SeeOther => String::from("303 See Other"),
            StatusCode::NotModified => String::from("304 Not Modified"),
            StatusCode::UseProxy => String::from("305 Use Proxy"),
            StatusCode::TemporaryRedirect => String::from("307 Temporary Redirect"),
            StatusCode::PermanentRedirect => String::from("308 Permanent Redirect"),
            StatusCode::BadRequest => String::from("400 Bad Request"),
            StatusCode::Unauthorized => String::from("401 Unauthorized"),
            StatusCode::PaymentRequired => String::from("402 Payment Required"),
            StatusCode::Forbidden => String::from("403 Forbidden"),
            StatusCode::NotFound => String::from("404 Not Found"),
            StatusCode::MethodNotAllowed => String::from("405 Method Not Allowed"),
            StatusCode::NotAcceptable => String::from("406 Not Acceptable"),
            StatusCode::ProxyAuthenticationRequired => {
                String::from("407 Proxy Authentication Required")
            }
            StatusCode::RequestTimeout => String::from("408 Request Timeout"),
            StatusCode::Conflict => String::from("409 Conflict"),
            StatusCode::Gone => String::from("410 Gone"),
            StatusCode::LengthRequired => String::from("411 Length Required"),
            StatusCode::PreconditionFailed => String::from("412 Precondition Failed"),
            StatusCode::PayloadTooLarge => String::from("413 Payload Too Large"),
            StatusCode::URITooLong => String::from("414 URI Too Long"),
            StatusCode::UnsupportedMediaType => String::from("415 Unsupported Media Type"),
            StatusCode::RangeNotSatisfiable => String::from("416 Range Not Satisfiable"),
            StatusCode::ExpectationFailed => String::from("417 Expectation Failed"),
            StatusCode::ImATeapot => String::from("418 I'm a teapot"),
            StatusCode::MisdirectedRequest => String::from("421 Misdirected Request"),
            StatusCode::UnprocessableEntity => String::from("422 Unprocessable Entity"),
            StatusCode::Locked => String::from("423 Locked"),
            StatusCode::FailedDependency => String::from("424 Failed Dependency"),
            StatusCode::TooEarly => String::from("425 Too Early"),
            StatusCode::UpgradeRequired => String::from("426 Upgrade Required"),
            StatusCode::PreconditionRequired => String::from("428 Precondition Required"),
            StatusCode::TooManyRequests => String::from("429 Too Many Requests"),
            StatusCode::RequestHeaderFieldsTooLarge => {
                String::from("431 Request Header Fields Too Large")
            }
            StatusCode::UnavailableForLegalReasons => {
                String::from("451 Unavailable For Legal Reasons")
            }
            StatusCode::InternalServerError => String::from("500 Internal Server Error"),
            StatusCode::NotImplemented => String::from("501 Not Implemented"),
            StatusCode::BadGateway => String::from("502 Bad Gateway"),
            StatusCode::ServiceUnavailable => String::from("503 Service Unavailable"),
            StatusCode::GatewayTimeout => String::from("504 Gateway Timeout"),
            StatusCode::HTTPVersionNotSupported => String::from("505 HTTP Version Not Supported"),
            StatusCode::VariantAlsoNegotiates => String::from("506 Variant Also Negotiates"),
            StatusCode::InsufficientStorage => String::from("507 Insufficient Storage"),
            StatusCode::LoopDetected => String::from("508 Loop Detected"),
            StatusCode::NotExtended => String::from("510 Not Extended"),
            StatusCode::NetworkAuthenticationRequired => {
                String::from("511 Network Authentication Required")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(StatusCode::Continue.to_string(), "100 Continue");
        assert_eq!(
            StatusCode::SwitchingProtocols.to_string(),
            "101 Switching Protocols"
        );
        assert_eq!(StatusCode::Processing.to_string(), "101 Processing");
        assert_eq!(StatusCode::EarlyHints.to_string(), "104 Early Hints");
        assert_eq!(StatusCode::OK.to_string(), "200 OK");
        assert_eq!(StatusCode::Created.to_string(), "201 Created");
        assert_eq!(StatusCode::Accepted.to_string(), "202 Accepted");
        assert_eq!(
            StatusCode::NonAuthoritativeInformation.to_string(),
            "203 Non-Authoritative Information"
        );
        assert_eq!(StatusCode::NoContent.to_string(), "204 No Content");
        assert_eq!(StatusCode::ResetContent.to_string(), "205 Reset Content");
        assert_eq!(
            StatusCode::PartialContent.to_string(),
            "206 Partial Content"
        );
        assert_eq!(StatusCode::MultiStatus.to_string(), "207 Multi-Status");
        assert_eq!(
            StatusCode::AlreadyReported.to_string(),
            "208 Already Reported"
        );
        assert_eq!(StatusCode::IMUsed.to_string(), "226 IM Used");
        assert_eq!(
            StatusCode::MultipleChoices.to_string(),
            "300 Multiple Choices"
        );
        assert_eq!(
            StatusCode::MovedPermanently.to_string(),
            "301 Moved Permanently"
        );
        assert_eq!(StatusCode::Found.to_string(), "302 Found");
        assert_eq!(StatusCode::SeeOther.to_string(), "303 See Other");
        assert_eq!(StatusCode::NotModified.to_string(), "304 Not Modified");
        assert_eq!(StatusCode::UseProxy.to_string(), "305 Use Proxy");
        assert_eq!(
            StatusCode::TemporaryRedirect.to_string(),
            "307 Temporary Redirect"
        );
        assert_eq!(
            StatusCode::PermanentRedirect.to_string(),
            "308 Permanent Redirect"
        );
        assert_eq!(StatusCode::BadRequest.to_string(), "400 Bad Request");
        assert_eq!(StatusCode::Unauthorized.to_string(), "401 Unauthorized");
        assert_eq!(
            StatusCode::PaymentRequired.to_string(),
            "402 Payment Required"
        );
        assert_eq!(StatusCode::Forbidden.to_string(), "403 Forbidden");
        assert_eq!(StatusCode::NotFound.to_string(), "404 Not Found");
        assert_eq!(
            StatusCode::MethodNotAllowed.to_string(),
            "405 Method Not Allowed"
        );
        assert_eq!(StatusCode::NotAcceptable.to_string(), "406 Not Acceptable");
        assert_eq!(
            StatusCode::ProxyAuthenticationRequired.to_string(),
            "407 Proxy Authentication Required"
        );
        assert_eq!(
            StatusCode::RequestTimeout.to_string(),
            "408 Request Timeout"
        );
        assert_eq!(StatusCode::Conflict.to_string(), "409 Conflict");
        assert_eq!(StatusCode::Gone.to_string(), "410 Gone");
        assert_eq!(
            StatusCode::LengthRequired.to_string(),
            "411 Length Required"
        );
        assert_eq!(
            StatusCode::PreconditionFailed.to_string(),
            "412 Precondition Failed"
        );
        assert_eq!(
            StatusCode::PayloadTooLarge.to_string(),
            "413 Payload Too Large"
        );
        assert_eq!(StatusCode::URITooLong.to_string(), "414 URI Too Long");
        assert_eq!(
            StatusCode::UnsupportedMediaType.to_string(),
            "415 Unsupported Media Type"
        );
        assert_eq!(
            StatusCode::RangeNotSatisfiable.to_string(),
            "416 Range Not Satisfiable"
        );
        assert_eq!(
            StatusCode::ExpectationFailed.to_string(),
            "417 Expectation Failed"
        );
        assert_eq!(StatusCode::ImATeapot.to_string(), "418 I'm a teapot");
        assert_eq!(
            StatusCode::MisdirectedRequest.to_string(),
            "421 Misdirected Request"
        );
        assert_eq!(
            StatusCode::UnprocessableEntity.to_string(),
            "422 Unprocessable Entity"
        );
        assert_eq!(StatusCode::Locked.to_string(), "423 Locked");
        assert_eq!(
            StatusCode::FailedDependency.to_string(),
            "424 Failed Dependency"
        );
        assert_eq!(StatusCode::TooEarly.to_string(), "425 Too Early");
        assert_eq!(
            StatusCode::UpgradeRequired.to_string(),
            "426 Upgrade Required"
        );
        assert_eq!(
            StatusCode::PreconditionRequired.to_string(),
            "428 Precondition Required"
        );
        assert_eq!(
            StatusCode::TooManyRequests.to_string(),
            "429 Too Many Requests"
        );
        assert_eq!(
            StatusCode::RequestHeaderFieldsTooLarge.to_string(),
            "431 Request Header Fields Too Large"
        );
        assert_eq!(
            StatusCode::UnavailableForLegalReasons.to_string(),
            "451 Unavailable For Legal Reasons"
        );
        assert_eq!(
            StatusCode::InternalServerError.to_string(),
            "500 Internal Server Error"
        );
        assert_eq!(
            StatusCode::NotImplemented.to_string(),
            "501 Not Implemented"
        );
        assert_eq!(StatusCode::BadGateway.to_string(), "502 Bad Gateway");
        assert_eq!(
            StatusCode::ServiceUnavailable.to_string(),
            "503 Service Unavailable"
        );
        assert_eq!(
            StatusCode::GatewayTimeout.to_string(),
            "504 Gateway Timeout"
        );
        assert_eq!(
            StatusCode::HTTPVersionNotSupported.to_string(),
            "505 HTTP Version Not Supported"
        );
        assert_eq!(
            StatusCode::VariantAlsoNegotiates.to_string(),
            "506 Variant Also Negotiates"
        );
        assert_eq!(
            StatusCode::InsufficientStorage.to_string(),
            "507 Insufficient Storage"
        );
        assert_eq!(StatusCode::LoopDetected.to_string(), "508 Loop Detected");
        assert_eq!(StatusCode::NotExtended.to_string(), "510 Not Extended");
        assert_eq!(
            StatusCode::NetworkAuthenticationRequired.to_string(),
            "511 Network Authentication Required"
        );
    }
}
