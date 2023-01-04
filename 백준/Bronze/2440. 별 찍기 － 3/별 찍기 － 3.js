const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
let str = '';
for (let i = 0; i < input; i++) {
  str += `${'*'.repeat(input - i)}\n`;
}
console.log(str.trim());