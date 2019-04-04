
    status string

    If the request was successful or not. Options: ok, error. In the case of error a code and message property will be populated.
    sources array[source]

    The results of the request.
        id string

        The identifier of the news source. You can use this with our other endpoints.
        name string

        The name of the news source
        description string

        A description of the news source
        url string

        The URL of the homepage.
        category string

        The type of news to expect from this news source.
        language string

        The language that this news source writes in.
        country string

        The country this news source is based in (and primarily writes about).
