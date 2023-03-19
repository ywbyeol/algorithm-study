console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split(' ')
    .reduce((a, b) => {
      return (+a + +b) * (+a - +b);
    }),
);