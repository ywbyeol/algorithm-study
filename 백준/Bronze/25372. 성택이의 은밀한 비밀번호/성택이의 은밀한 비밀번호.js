const fs = require('fs');
const [N, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < N; i++) {
  if (input[i].length >= 6 && input[i].length <= 9) {
    str += 'yes\n';
    continue;
  }
  str += 'no\n';
}
console.log(str.trim());