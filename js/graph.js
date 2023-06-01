document.getElementsByTagName("html")[0].className += " jsEnabled";

var boxsize = 60;
var zoomIn = true;
var emojimode = true;
var nightmode = true;

// Assume that there is a js object with name 'graph_data' already present...

const Graph = ForceGraph()(document.getElementById("graph"))
  .warmupTicks(60)
  .cooldownTicks(50)
  .nodeId("id")
  .nodeVal("val")
  .nodeLabel("title")
  .nodeAutoColorBy("id")
  .nodeCanvasObject((node, ctx, globalScale) => {
    let label = emojimode ? node.icon : node.title;

    const fontSize = 16;
    ctx.font = `${fontSize}px Sans-Serif`;

    //const textWidth = ctx.measureText(node_dimensions).width;
    const textWidth = ctx.measureText(label).width;
    const bckgDimensions = [textWidth, fontSize].map((n) => n + fontSize * 0.2); // some padding

    ctx.fillStyle = "rgba(255, 255, 255, 0.0)";

    ctx.fillRect(
      node.x - bckgDimensions[0] / 2,
      node.y - bckgDimensions[1] / 2,
      ...bckgDimensions
    );

    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillStyle = node.color;
    ctx.fillText(label, node.x, node.y);

    node.__bckgDimensions = bckgDimensions; // to re-use in nodePointerAreaPaint
  })
  .nodePointerAreaPaint((node, color, ctx) => {
    ctx.fillStyle = color;
    const bckgDimensions = node.__bckgDimensions;
    bckgDimensions &&
      ctx.fillRect(
        node.x - bckgDimensions[0] / 2,
        node.y - bckgDimensions[1] / 2,
        ...bckgDimensions
      );
  })
  .linkSource("source")
  .linkTarget("target")
  .linkWidth(1.3)
  .linkColor(() => (nightmode ? "white" : "black"))
  .linkDirectionalArrowLength(2)
  .linkAutoColorBy((d) => {
    d.source.tag;
  })
  .onNodeClick((node) => {
    if (node.id == "day") {
      document.getElementsByClassName("logo")[0].style.color = "black";

      document.body.style.background =
        "#D3CCE3"; /* fallback for old browsers */
      document.body.style.background =
        "-webkit-linear-gradient(to bottom, #E9E4F0, #D3CCE3)"; /* Chrome 10-25, Safari 5.1-6 */
      document.body.style.background =
        "linear-gradient(to bottom, #E9E4F0, #D3CCE3)"; /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */

      nightmode = false;
    } else if (node.id == "night") {
      document.getElementsByClassName("logo")[0].style.color = "white";

      document.body.style.background =
        "#0F2027;"; /* fallback for old browsers */
      document.body.style.background =
        "-webkit-linear-gradient(to bottom, #2C5364, #203A43, #0F2027)"; /* Chrome 10-25, Safari 5.1-6 */
      document.body.style.background =
        "linear-gradient(to bottom, #2C5364, #203A43, #0F2027)"; /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */

      nightmode = true;
    } else if (node.id == "emoji") {
      emojimode = !emojimode;
    } else if (node.id == "resume") {
      location.href = "iain_maitland_resume.pdf";
    } else if (node.id == "photos") {
      location.href =
        "https://photos.google.com/share/AF1QipPgG8AHI2l56B7gtrN2GKYQjrJW05wOy1GPVqAGj2eR-RtTuoRFTFdCSN0Z13Zryw?key=U0VsTGFNSkM5VVBOeVFOajg4RXpRNFc2Q25KV1RR";
    } else if (node.id == "notebooks") {
      location.href = "https://imaitland.github.io/notebooks/lab/index.html";
    } else {
      window.location.href = node.id;
    }
  })
  //.d3Force('charge', null)
  .onEngineStop(() => {
    if (zoomIn) {
      Graph.zoomToFit(1000, 80);
      // Don't zoomin again.
      zoomIn = false;
      // remove the box to prevent bugs when dragging.
    }
  })

  // Contain nodes to a box.
  .graphData(graph_data);
