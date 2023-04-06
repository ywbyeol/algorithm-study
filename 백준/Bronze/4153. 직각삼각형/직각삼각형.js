console.log(
  `${require('fs').readFileSync('/dev/stdin')}`.trim().split('\n').slice(0, -1)
    .map(i => {
      const [a, b, c] = i.split(' ').sort((x, y) => +x - +y);
      return (+a) ** 2 + (+b) ** 2 === (+c) ** 2 ? 'right' : 'wrong';
    }).join('\n')
);