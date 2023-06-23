console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split(/\s+/)
    .slice(1)
    .map(Number)
    .sort((a, b) => a - b)
    .slice(1, -1)
    .reduce((a, b) => a + b),
);