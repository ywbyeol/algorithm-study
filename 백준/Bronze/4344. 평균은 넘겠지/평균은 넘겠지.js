console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(x => {
      const [n, ...el] = x.split(' ').map(Number);
      return `${(
        (el.filter(y => y > el.reduce((a, b) => a + b) / n).length / n) *
        100
      ).toFixed(3)}%`;
    })
    .join('\n')
);