console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(x => x[0].toUpperCase() + x.slice(1))
    .join('\n'),
);