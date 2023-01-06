const fs = require('fs');
const [n, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < n; i++) {
  const [A, B] = input[i].split(',').map(Number);
  str += `${A + B}\n`;
}
console.log(str.trim());