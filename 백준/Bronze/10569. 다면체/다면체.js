console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(x => {
      const [V, E] = x.split(' ');
      return -+V + +E + 2;
    })
    .join('\n'),
);