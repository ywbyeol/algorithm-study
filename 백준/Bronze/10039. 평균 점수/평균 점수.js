const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let sum = 0;
for (let i = 0; i < 5; i++) {
  if (input[i] < 40) {
    sum += 40;
    continue;
  }
  sum += input[i];
}
console.log(sum / 5);