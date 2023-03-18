console.log(
  require('fs')
    .readFileSync('/dev/stdin', 'utf8')
    .trim()
    .split('\n')
    .reduce((a, b) => {
      return +a + +b;
    }, 0),
);