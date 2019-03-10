use custom_error::custom_error;

custom_error! {pub NewsApiError
    InvalidParameterCombinationError = "The source parameter cannot be mixed with country or category",
    UndefinedUrlError = "The NewsApi URL hasn't been constructed properly",
    BadRequest = "Request did not return success - failing with a bad message!",
    ReqwestError{source: reqwest::Error} = "boom!",
}

