const fs = require('fs');
console.log(
  fs
    .readFileSync('/dev/stdin')
    .toString()
    .trim()
    .split('\n')
    .slice(0, -1)
    .map(item => {
      const num = Number(item);
      return (num * (num + 1)) / 2;
    })
    .join('\n'),
);