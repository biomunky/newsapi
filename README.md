# The News API Rust Library

[![Build Status](https://github.com/biomunky/newsapi/workflows/CI/badge.svg)](https://github.com/biomunky/newsapi/actions?query=workflow%3ACI)

- [The News API Rust Library](#the-news-api-rust-library)
  - [Breaking changes**](#breaking-changes)
  - [Summary](#summary)
  - [Notes](#notes)
  - [Examples](#examples)
  - [License](#license)
  - [Disclaimer](#disclaimer)

[The News API](https://newsapi.org/) allows you to get breaking news headlines, and search for articles from over 30,000 news sources and blogs.

## Breaking changes**

- Version 0.5.x renames the `Client` struct to `NewsAPIClient`. See [examples](examples/) for demonstrated usage.

- Version 0.4.x introduces async fetch. While synchronous functionality is retained, the relevant functions have been renamed. Please check [examples](examples/) for further details. The core change is `send` has been replaced with `send_async` and `send_sync` for the asynchronous and synchronous variants respectively.

## Summary

All you need is an API key to

- Search live top headlines or search all articles
- New articles available with 15 minute delay
- And Search articles up to a month old
- Get short article snippet or descriptions

The following limits apply

- 1,000 requests per day
- No extra requests available
- No uptime SLA
- Basic support
- NewsAPI.org attribution required

See [here for pricing](https://newsapi.org/pricing)

## Notes

There are three endpoints

1. [Top Headlines](https://newsapi.org/docs/endpoints/top-headlines)
2. [Everything](https://newsapi.org/docs/endpoints/everything)
3. [Sources](https://newsapi.org/docs/endpoints/sources)

Top Headlines and Everything endpoints are wrapped by an Article struct and Sources by a Source struct. Both currently provide limited functionality.

## Examples

As described in [The Cargo Book](https://doc.rust-lang.org/cargo/guide/project-layout.html) the project has some simple examples in [examples/](examples/). These can be run via cargo after you've exported your NEWSAPI_KEY

```shell
export NEWSAPI_KEY=5h79off128957edb3179y5da1nb36y9e
```

To list all examples:

```shell
cargo run --example
```

to run a specific example

```shell
cargo run --example get_sources_async
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Disclaimer

We have no affiliation with the [News API](https://github.com/News-API-gh) team.
