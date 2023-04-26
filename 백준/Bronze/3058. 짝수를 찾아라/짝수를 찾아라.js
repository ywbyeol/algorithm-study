console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(x => {
      const i = x
        .split(' ')
        .map(Number)
        .filter(v => v % 2 === 0);
      return `${i.reduce((a, b) => a + b)} ${Math.min(...i)}`;
    })
    .join('\n'),
);