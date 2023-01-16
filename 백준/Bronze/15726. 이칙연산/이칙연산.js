const fs = require('fs');
const [a, b, c] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
const result1 = Math.floor((a * b) / c);
const result2 = Math.floor((a / b) * c);
console.log(result1 > result2 ? result1 : result2);