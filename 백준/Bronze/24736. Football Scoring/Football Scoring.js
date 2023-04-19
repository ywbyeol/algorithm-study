console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .map(x => {
      const [T, F, S, P, C] = x.split(' ');
      return +T * 6 + +F * 3 + +S * 2 + +P + +C * 2;
    })
    .join(' '),
);