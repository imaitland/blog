# Blog

Welcome to my lil static site generator. It's used to create my [blog](https://iainmaitland.com).

> :dromedary_camel: Things could break at any time, use at your own risk! I made this static site generator to get a better grip on using Rust for web development, and for fun.

It includes a build command: `cargo run -- --build` which generates the static assets in `/dist`. And a dev server command, `cargo run -- --port 8008`, 

To get started you just need to add markdown files in the `/md` directory, making sure their frontmatter conforms to the `Frontmatter` struct. The `Graph` struct is generated to represent how all the files in `/md` are related and can be used to create a site index or contents page, in my case I am generating a `force-graph` at `/`.

When I say "how all the files are related", if one file contains a md link, to another, an edge for the graph representation of the site is generated.

Further development is needed to support external links, currently they require a node of their own and an explicit redirect, e.g. the Resume and Notebooks nodes.

All this is made possible by the following dependencies (the JS dependencies are optional and are only used to generate the interactive graph at `/`):

- **Server**: [rouille](https://github.com/tomaka/rouille)
- **Markdown Parsing**: [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
- **HTML Templates**: [maud](https://github.com/lambda-fairy/maud)
- **Graph Vizualization**: [force-graph](https://github.com/vasturiano/force-graph)
- **Animations**: [animejs](https://github.com/juliangarnier/anime/)
- **Syntax Highlighting**: [highlight.js](https://github.com/highlightjs/highlight.js)

## Install and run locally

To get the blog running locally you can run the following commands.

### Install and Run
```
yarn build
cargo run -- --port 8000
```

With the site running locally, (defaults to `port 8080`) you can add markdown files to the `/md` directory, make sure their frontmatter conforms to the `Frontmatter` struct defined in `render.rs`.

### Generate Static Site
To generate a static build run:
```
cargo run -- --build
```
The files in the newly created `dist` directory represent your site.

## Next steps.
- [ ] Testss
- [ ] Enable drafts.
- [x] Post Social / OpenGraph meta / header.
- [x] Create a static site build command, and host on s3 / netlify / github pages or similar.
- [ ] Write an image hoster service to make it easy to embed responsive images.
