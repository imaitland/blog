# Blog

This is the code that generates my blog. I suppose you could call it a static site generator (SSG) because the build happens at a different stage than the usual client requests...

![](https://cdn.netlify.com/b0cd7be20ba718c92b5da007a109a89122f6791a/7824d/img/blog/ssg-host-flow.png)

To achieve this - in an ergonomic way - this software has two modes one is a `dev server` that can be run locally as you write posts and do the basic plumbing needed to get a satisfying result.

The other is the build process, that generates the static html, which can be hosted on a static service, like S3, Github pages, Netlify or similar. 

It's worth noting that this particular SSG is custom to me and YMMV!

## Mode 1: Dev Server:

Start the server using `cargo run`.

Markdown files go into the `md` directory and make sure they have some frontmatter that conforms to the schema.

All the markdown files in this directory are parsed using [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)

The output is routed through [maud](https://github.com/lambda-fairy/maud) which adds templating to the raw html output.

As this is happening we build a graph of the posts, each post is a node and each link between posts is an edge.

Finally [rouille](https://github.com/tomaka/rouille) manages client requests, serving html as required.

At the index route '/' we serve the graph of all the posts, making it look pretty and interactive with [force-graph](https://github.com/vasturiano/force-graph).

## Mode 2: Generating your static site:
Once you have a satisfactory result using the dev server, you can generate a static representation of the site:
`cargo run --static-site`
This will generate html assets in a newly created `dist` directory which can then be hosted with your service of choice.

## Next steps.
I plan on writing an image hoster service, that can handle requests for images of different sizes and crop-ratios. This will make embedding responsive images in blog posts easier.


