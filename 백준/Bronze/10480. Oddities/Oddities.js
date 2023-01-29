const fs = require('fs');
const [T, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .split('\n')
  .map(Number);
let answer = '';
for (let i = 0; i < T; i++) {
  answer += input[i] % 2 === 0 ? `${input[i]} is even\n` : `${input[i]} is odd\n`;
}
console.log(answer.trim());
