# The News API

[The News API](https://newsapi.org/) allows you to get breaking news headlines, and search for articles from over 30,000 news sources and blogs.

All you need is an API key to

* Search live top headlines or search all articles
* New articles available with 15 minute delay
* And Search articles up to a month old
* Get short article snippet or descriptions

The following limits apply

* 1,000 requests per day
* No extra requests available
* No uptime SLA
* Basic support
* NewsAPI.org attribution required

See [here for pricing](https://newsapi.org/pricing)

## Notes

### Three endpoints

1. [Top Headlines](https://newsapi.org/docs/endpoints/top-headlines)
2. [Everything](https://newsapi.org/docs/endpoints/everything)
3. [Sources](https://newsapi.org/docs/endpoints/sources)

### General Structure

```json
{
    "status": "ok",
    "totalResults": 36,
    -
    "articles": [
        -
        {
            -
            "source": {
                "id": null,
                "name": "Washingtonexaminer.com"
            },
            "author": "https://www.washingtonexaminer.com/author/becket-adams",
            "title": "The New York Times is trying hard to clean up after AOC's Green New Deal mess - Washington Examiner",
            "description": "It’s one thing for a newsroom to bend over backward and puff up a member of Congress. It’s another thing entirely to help that lawmaker push a lie.",
            "url": "https://www.washingtonexaminer.com/opinion/the-new-york-times-is-trying-hard-to-clean-up-after-aocs-green-new-deal-mess",
            "urlToImage": "https://mediadc.brightspotcdn.com/dims4/default/a1ee1e3/2147483647/strip/true/crop/2290x1202+0+0/resize/1200x630!/quality/90/?url=https%3A%2F%2Fmediadc.brightspotcdn.com%2F92%2Fab%2F23f9b7b84f76b964c4bd53007575%2Faoc-nyt.jpg",
            "publishedAt": "2019-03-05T20:59:04Z",
            "content": "Its one thing for a newsroom to bend over backward and puff up a member of Congress. Its another thing entirely to help that lawmaker push a lie. The New York Times did exactly that when it published a servile bit of spin this week in service of Rep. Alexand… [+3153 chars]"
        }
    ]
}
```

## Examples
As described in [The Cargo Book](https://doc.rust-lang.org/cargo/guide/project-layout.html) the project has some simple examples in examples/. These can be run via cargo after you've exported you NEWSAPI_KEY

```
export NEWSAPI_KEY=5h79off128957edb3179y5da1nb36y9e
```

To list all examples:
```
cargo run --example
```

to run a specific example
```
cargo run --example get_sources
```
