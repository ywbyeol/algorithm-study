const fs = require('fs');
const [N, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < N; i++) {
  const arr = input[i].split('').map(String);
  str += `${arr[0]}${arr.at(-1)}\n`;
}
console.log(str);