# Shorty - A little URL shortener in Rust

Mostly a personal project to test fly.io but feel free to extend.

## Available Routes

`/stats` -> Shows a list of redirects and how many times they've been hit

`/add` -> Add a new redirect, expects a JSON body with `short` `long` and `secret`

`/del` -> remove an existing redirect, expects a JSON body with `short` and `secret`

For `add` and `del` the parameters are:

- `short` is the short code that will go at the end of `<baseurl>/a/...`
- `long` is the URL to redirect to
- `secret` is the contents of the environment veriable ADD_SECRET used to restrict who can add new links


`/s/<short>` -> Redirects to the associated `long` in the database or to `/lost` if there is no matching short code.

Visiting a short link will increment the hits count by 1.
Caching of responses might mean this doesn't always happen but it should be okay as rough stats.


## Known Limitations

- Everything is a text response rather than HTML, intentional for now
- You can add two redirects that use the same short code
