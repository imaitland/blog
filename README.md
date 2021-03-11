# Blog

Welcome to my custom static site generator. It's used to create my [blog](https://iainmaitland.com).

Locally it's a simple webserver, running the following stack:

- **Server**: [rouille](https://github.com/tomaka/rouille)
- **Markdown Parsing**: [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
- **Templates**: [maud](https://github.com/lambda-fairy/maud)
- **Graph Vizualization**: [force-graph](https://github.com/vasturiano/force-graph)
- **Animations**: [animejs](https://github.com/juliangarnier/anime/)
- **Syntax Highlighting**: [highlight.js](https://github.com/highlightjs/highlight.js)

It also features the ability to generate a static version of the site, for cheap and easy hosting on your provider of choice.

## Install and run locally

To get the blog running locally you can run the following commands.

```
yarn build
cargo run
```

With the site running locally, defaults to `port 8080` you can add markdown files to the `/md` directory, make sure their frontmatter conforms to the struct defined in `render.rs`.

To generate a static build run:
```
cargo run --ssg
```
The files in the newly created `dist` directory represent your site.

## Next steps.
- [ ] Enable drafts.
- [x] Create a static site build command, and host on s3 / netlify / github pages or similar.
- [ ] Create an admin mode, to create posts and deploy a static build of the site from the browser.
- [ ] Write an image hoster service to make it easy to embed responsive images.

