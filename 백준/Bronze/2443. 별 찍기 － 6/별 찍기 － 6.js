const fs = require('fs');
const input = Number(fs.readFileSync('/dev/stdin').toString().trim());
let str = '';
for (let i = 0; i < input; i++) {
  str += `${' '.repeat(i)}${'*'.repeat((input - 1) * 2 + 1 - i * 2)}\n`;
}
console.log(str.trimEnd());