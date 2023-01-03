const fs = require('fs');
const [n, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let answer = '';
for (let i = 0; i < n; i++) {
  answer += `${'='.repeat(input[i])}\n`;
}
console.log(answer.trim());