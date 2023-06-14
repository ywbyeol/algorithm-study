console.log(
  `${require('fs').readFileSync(0)}`
    .split('\n')[1]
    .split('')
    .reduce((a, b) => a + (b.charCodeAt(0) - 64), 0),
);