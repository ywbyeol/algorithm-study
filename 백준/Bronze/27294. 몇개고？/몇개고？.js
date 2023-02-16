const fs = require('fs');
const [T, S] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log(S === 1 || !(T >= 12 && T <= 16) ? 280 : 320);