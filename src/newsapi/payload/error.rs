HTTP status codes summary

    200 - OK. The request was executed successfully.
    400 - Bad Request. The request was unacceptable, often due to a missing or misconfigured parameter.
    401 - Unauthorized. Your API key was missing from the request, or wasn't correct.
    429 - Too Many Requests. You made too many requests within a window of time and have been rate limited. Back off for a while.
    500 - Server Error. Something went wrong on our side.

Error codes

When an HTTP error is returned we populate the code and message properties in the response containing more information. Here are the possible options:

    apiKeyDisabled - Your API key has been disabled.
    apiKeyExhausted - Your API key has no more requests available.
    apiKeyInvalid - Your API key hasn't been entered correctly. Double check it and try again.
    apiKeyMissing - Your API key is missing from the request. Append it to the request with one of these methods.
    parameterInvalid - You've included a parameter in your request which is currently not supported. Check the message property for more details.
    parametersMissing - Required parameters are missing from the request and it cannot be completed. Check the message property for more details.
    rateLimited - You have been rate limited. Back off for a while before trying the request again.
    sourcesTooMany - You have requested too many sources in a single request. Try splitting the request into 2 smaller requests.
    sourceDoesNotExist - You have requested a source which does not exist.
    unexpectedError - This shouldn't happen, and if it does then it's our fault, not yours. Try the request again shortly.
