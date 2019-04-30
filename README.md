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

There are three endpoints

1. [Top Headlines](https://newsapi.org/docs/endpoints/top-headlines)
2. [Everything](https://newsapi.org/docs/endpoints/everything)
3. [Sources](https://newsapi.org/docs/endpoints/sources)

Top Headlines and Everything endpoints are wrapped by an Article struct and Sources by a Source struct. Both currently provide limited functionality.

## Examples
As described in [The Cargo Book](https://doc.rust-lang.org/cargo/guide/project-layout.html) the project has some simple examples in examples/. These can be run via cargo after you've exported your NEWSAPI_KEY

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


## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


## Disclaimer

We have no affiliation with the [News API](https://github.com/News-API-gh) team.
