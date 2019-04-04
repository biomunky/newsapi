
    status string

    If the request was successful or not. Options: ok, error. In the case of error a code and message property will be populated.
    totalResults int

    The total number of results available for your request.
    articles array[article]

    The results of the request.
        source object

        The identifier id and a display name name for the source this article came from.
        author string

        The author of the article
        title string

        The headline or title of the article.
        description string

        A description or snippet from the article.
        url string

        The direct URL to the article.
        urlToImage string

        The URL to a relevant image for the article.
        publishedAt string

        The date and time that the article was published, in UTC (+000)
        content string

        The unformatted content of the article, where available. This is truncated to 260 chars for Developer plan users.
