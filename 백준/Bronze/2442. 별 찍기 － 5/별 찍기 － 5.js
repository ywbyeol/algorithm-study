const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
let str = '';
let star = '*';
for (let i = 1; i <= input; i++) {
  str += `${' '.repeat(input - i)}${star}\n`;
  star += '**';
}
console.log(str.trimEnd());