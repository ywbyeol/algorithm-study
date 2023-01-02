const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let sum = 1;
for (let i = 0; i < input.length; i++) {
  sum += input[i];
}
console.log(sum);