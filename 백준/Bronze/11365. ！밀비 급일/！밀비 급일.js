console.log(
  `${require('fs').readFileSync('/dev/stdin')}`
    .trim()
    .split('\n')
    .slice(0, -1)
    .map(x => x.split('').reverse().join(''))
    .join('\n'),
);