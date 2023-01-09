const fs = require('fs');
const S = fs.readFileSync('/dev/stdin').toString().trim();
const alphabet = 'abcdefghijklmnopqrstuvwxyz';
let str = '';
for (let i = 0; i < 26; i++) {
  str += `${S.indexOf(alphabet[i])} `;
}
console.log(str.trim());