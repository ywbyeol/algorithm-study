const fs = require('fs');
const [n, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < Number(n); i++) {
  const [X, Y] = input[i].split(' ').map(Number);
  str += X < Y ? 'NO BRAINS\n' : 'MMM BRAINS\n';
}
console.log(str.trim());