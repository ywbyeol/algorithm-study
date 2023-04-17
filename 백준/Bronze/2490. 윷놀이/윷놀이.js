console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .map(
      x => ['D', 'C', 'B', 'A', 'E'][x.split(' ').reduce((a, b) => a + +b, 0)],
    )
    .join('\n'),
);