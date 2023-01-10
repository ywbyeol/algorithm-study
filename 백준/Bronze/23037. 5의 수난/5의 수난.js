const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('')
  .map(Number);
let sum = 0;
input.forEach((el) => {
  sum += el ** 5;
});
console.log(sum);