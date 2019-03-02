# Snowfall

Snowfall is a bookmark manager based off [Shiori](https://github.com/RadhiFadlillah/shiori) and [Raindrop.io](https://raindrop.io).

## Features

- Multiple user accounts, each with their own collections, bookmarks, and tags.
- Sort bookmarks by collection and tag them for more categorization
- Powered by a GraphQL API, with built in Relay support.
- Multiple database backend support:
  - MySQL
  - PostgreSQL
  - SQLite (default)

Planned:

- A command line interface, with support for talking to a running instance of Snowfall.
- A browser extension for Firefox and Chrome.
- Download a local version of the website to be viewed at any time.
  - Will even download and save videos and PDFs.
- Importing and exporting of bookmark to different bookmark formats.
- Make a bookmark public to allow people without an account to view them.
- A comic/manga and story reader, maybe even user made extensions for it.
- SSL/TLS for the web frontend and the MySQL and PostgreSQL database backends.

Might Implement:

- Allow the use to annotate articles.
- Download the website as a WARC instead of raw HTML.
- Export collections, bookmarks, and tags as a static site.

## Installation

If you don't want to change the running port or database backend, installing Snowfall is as easy as downloading the correct version for your device and running it.

## Building

To build Snowfall you'll need a C compiler and Rust installed. If you're going to be using SSL you'll need OpenSSL installed.

As long as your environment is setup correctly, it should be as easy as `cargo build ---release`.