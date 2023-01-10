const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('');
const alphabet = 'abcdefghijklmnopqrstuvwxyz';
let str = '';
for (let i = 0; i < 26; i++) {
  let count = 0;
  for (let j = 0; j < input.length; j++) {
    if (input[j] === alphabet[i]) {
      count++;
    }
  }
  str += `${count} `;
}
console.log(str.trim());