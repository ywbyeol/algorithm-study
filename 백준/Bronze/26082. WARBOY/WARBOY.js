const fs = require('fs');
const [a, b, c] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log((3 * b * c) / a);