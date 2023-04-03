console.log(
  `${require('fs').readFileSync('/dev/stdin')}`
    .trim()
    .split('-')
    .reduce((a, b) => a + b[0], ''),
);