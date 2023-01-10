const fs = require('fs');
const [a, b] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log(a - a * (b / 100) >= 100 ? 0 : 1);