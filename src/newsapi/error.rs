use custom_error::custom_error;

custom_error! { pub NewsApiError
    InvalidParameterCombinationError = "The source parameter cannot be mixed with country or category",
    UndefinedUrlError = "Error constructing newsapi URL",
    GenericError{code: u16, message: String} = "GenericError: {code} => {message}",
    BadRequest{code: u16, message: String} = "BadRequest: {code} => {message}",
    Unauthorized{code: u16, message: String} = "Unauthorized: {code} => {message}",
    TooManyRequests{code: u16, message: String} = "TooManyRequests: {code} => {message}",
    ServerError{code: u16, message: String} = "ServerError: {code} => {message}",
    ReqwestError{source: reqwest::Error} = "Reqwest Failure!",
}
