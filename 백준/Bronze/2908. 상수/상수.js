const fs = require('fs');
console.log(
  Math.max(
    ...fs
      .readFileSync('/dev/stdin')
      .toString()
      .trim()
      .split('')
      .reverse()
      .join('')
      .split(' ')
      .map(Number),
  ),
);