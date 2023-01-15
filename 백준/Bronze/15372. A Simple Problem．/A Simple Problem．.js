const fs = require('fs');
const [T, ...N] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let str = '';
for (let i = 0; i < T; i++) {
  str += `${N[i] ** 2}\n`;
}
console.log(str.trim());