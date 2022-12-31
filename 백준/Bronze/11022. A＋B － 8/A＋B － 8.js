const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let str = '';
for (let i = 1; i < Number(input[0]) + 1; i++) {
  const el = input[i].split(' ');
  str += `Case #${i}: ${el[0]} + ${el[1]} = ${Number(el[0]) + Number(el[1])}\n`;
}
console.log(str.trim());