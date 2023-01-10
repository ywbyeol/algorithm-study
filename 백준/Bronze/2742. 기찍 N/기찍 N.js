const fs = require('fs');
const N = fs.readFileSync('/dev/stdin').toString().trim();
let str = '';
for (let i = 0; i < N; i++) {
  str += `${N - i}\n`;
}
console.log(str.trim());