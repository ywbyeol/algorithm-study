const fs = require('fs');
const [T, ...input] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
let answer = '';
for (let i = 0; i < Number(T); i++) {
  const [R, S] = input[i].split(' ');
  let str = '';
  for (let j = 0; j < S.length; j++) {
    str += S[j].repeat(Number(R));
  }
  answer += `${str}\n`;
}
console.log(answer.trim());