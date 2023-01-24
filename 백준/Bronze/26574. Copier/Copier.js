const fs = require('fs');
const [n, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let str = '';
for (let i = 0; i < n; i++) {
  str += `${input[i]} ${input[i]}\n`;
}
console.log(str.trim());