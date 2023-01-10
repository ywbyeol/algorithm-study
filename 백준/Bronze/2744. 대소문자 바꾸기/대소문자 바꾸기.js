const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('');
let str = '';
for (let i = 0; i < input.length; i++) {
  const el = input[i];
  if (el === el.toLowerCase()) {
    str += el.toUpperCase();
    continue;
  }
  str += el.toLowerCase();
}
console.log(str);