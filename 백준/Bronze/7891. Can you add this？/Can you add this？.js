const fs = require('fs');
const [n, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < Number(n); i++) {
  const [x, y] = input[i].split(' ').map(Number);
  str += `${x + y}\n`;
}
console.log(str.trim());