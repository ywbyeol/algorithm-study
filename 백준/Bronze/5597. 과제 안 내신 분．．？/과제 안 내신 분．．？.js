const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
input.sort((a, b) => a - b);
let str = '';
for (let i = 1; i <= 30; i++) {
  if (!input.includes(i)) {
    str += `${i}\n`;
  }
}
console.log(str.trim());