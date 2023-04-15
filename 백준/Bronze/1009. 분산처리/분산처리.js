console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(x => {
      const [a, b] = x.split(' ');
      let pow = 1;
      for (let i = 0; i < b; i++) pow = (pow * a) % 10;
      return pow === 0 ? 10 : pow;
    })
    .join('\n'),
);