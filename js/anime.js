anime({
    targets: '.reveal',
    translateX: function(el) {
        return el.getAttribute('data-x');
    },
    translateY: function(el) {
        return el.getAttribute('data-y');
    },
    easing: 'cubicBezier(.5, .05, .1, .3)',
    direction: 'reverse',
    duration: 2000,
    opacity: 0,
});

anime({
    targets: '.revealspring',
    translateX: function(el) {
        return el.getAttribute('data-x');
    },
    translateY: function(el) {
        return el.getAttribute('data-y');
    },
    easing: 'easeInElastic(10, 0.75)',
    direction: 'reverse',
    duration: 2000,
    opacity: 0,
  })
