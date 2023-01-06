const fs = require('fs');
const [n1, k1, n2, k2] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log(n1 * k1 + n2 * k2);