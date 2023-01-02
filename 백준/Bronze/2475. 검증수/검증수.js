const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
let sum = 0;
for (let i = 0; i < 5; i++) {
  sum += Math.pow(input[i], 2);
}
console.log(sum % 10);