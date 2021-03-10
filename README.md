# Blog

Welcome to my blog, it's a pretty simple webserver, running the following stack:

- **Server**: [rouille](https://github.com/tomaka/rouille)
- **Markdown Parsing**: [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
- **Templates**: [maud](https://github.com/lambda-fairy/maud)
- **Graph Vizualization**: [force-graph](https://github.com/vasturiano/force-graph)
- **Animations**: [animejs](https://github.com/juliangarnier/anime/)
- **Syntax Highlighting**: [highlight.js](https://github.com/highlightjs/highlight.js)

## Next steps.
- [ ] Enable drafts.
- [ ] Create a static site build command, and host on s3 / netlify / github pages or similar.
- [ ] Create an admin mode, to create posts and deploy a static build of the site from the browser.
- [ ] Write an image hoster service to make it easy to embed responsive images.

## Install and run locally
```
yarn build
cargo run
``
