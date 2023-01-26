const fs = require('fs');
const [T, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let str = '';
for (let i = 0; i < Number(T); i++) {
  const arr = input[i]
    .split(' ')
    .map(Number)
    .sort((a, b) => a - b);
  arr.shift();
  arr.pop();
  if (arr[2] - arr[0] >= 4) {
    str += 'KIN\n';
  } else {
    str += `${arr.reduce((pre, cur) => {
      return pre + cur;
    }, 0)}\n`;
  }
}
console.log(str.trim());