const fs = require('fs');
const [N, M] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log(Math.min(Math.floor(N / 2), Math.floor(M / 2)));