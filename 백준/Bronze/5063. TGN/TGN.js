console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .split('\n')
    .slice(1)
    .map(x => {
      const [r, e, c] = x.split(' ');
      return +r < +e - +c
        ? 'advertise'
        : +r === +e - +c
        ? 'does not matter'
        : 'do not advertise';
    })
    .join('\n'),
);
