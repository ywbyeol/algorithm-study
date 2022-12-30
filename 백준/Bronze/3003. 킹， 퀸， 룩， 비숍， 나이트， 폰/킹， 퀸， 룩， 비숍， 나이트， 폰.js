const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split(' ');
const right = [1, 1, 2, 2, 2, 8];
let str = '';
for (let i = 0; i < 6; i++) {
  str += ' ' + String(right[i] - input[i]);
}
console.log(str.trim());