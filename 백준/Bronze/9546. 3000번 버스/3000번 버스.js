console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(v => {
      let p = 0;
      for (let i = 0; i < +v; i++) p = (p + 0.5) * 2;
      return p;
    })
    .join('\n')
);