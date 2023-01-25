const fs = require('fs');
const [A, B, C] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number)
  .sort((a, b) => a - b);
let answer;
if (C === A + B) {
  answer = 1;
} else if (C === A * B) {
  answer = 2;
} else {
  answer = 3;
}
console.log(answer);