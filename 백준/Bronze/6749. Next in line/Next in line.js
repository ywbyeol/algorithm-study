const fs = require('fs');
const [a, b] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
console.log(b + b - a);