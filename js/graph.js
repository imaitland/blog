fetch('/graph').then(res => res.json()).then(data => {
    const Graph = ForceGraph()
    (document.getElementById('graph'))
      .graphData(data)
      .nodeId('id')
      .nodeVal('val')
      .nodeLabel('title')
      .nodeAutoColorBy('id')
      .nodeCanvasObject((node, ctx, globalScale) => {
        const label = node.icon;
        let fontSize = 40/globalScale;
        ctx.font = `${fontSize}px Sans-Serif`;
        const textWidth = ctx.measureText(label).width;
        const bckgDimensions = [textWidth, fontSize].map(n => n + fontSize * 0.2); // some padding

        ctx.fillStyle = 'rgba(255, 255, 255, 0.0)';
        ctx.fillRect(node.x - bckgDimensions[0] / 2, node.y - bckgDimensions[1] / 2, ...bckgDimensions);

        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillStyle = node.color;
        ctx.fillText(label, node.x, node.y);
        //ctx.fillText("HELLO", node.x, node.y);
        fontSize = 10/globalScale;
        ctx.font = `${fontSize}px Sans-Serif`;
        ctx.fillText(node.title, node.x-2.5, node.y+8);

        node.__bckgDimensions = bckgDimensions; // to re-use in nodePointerAreaPaint
      })
      .nodePointerAreaPaint((node, color, ctx) => {
        ctx.fillStyle = color;
        const bckgDimensions = node.__bckgDimensions;
        bckgDimensions && ctx.fillRect(node.x - bckgDimensions[0] / 2, node.y - bckgDimensions[1] / 2, ...bckgDimensions);
      })
      .linkSource('source')
      .linkTarget('target')
      .linkWidth(1)
      .linkAutoColorBy(d => {
        d.source.tag
      })
      .onNodeClick(node => {
        window.location.href = node.id;
      });
  });