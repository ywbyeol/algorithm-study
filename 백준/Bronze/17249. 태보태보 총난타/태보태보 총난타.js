console.log(
  `${require('fs').readFileSync(0)}`
    .split('(^0^)')
    .map(v => v.split('').filter(x => x === '@').length)
    .join(' '),
);