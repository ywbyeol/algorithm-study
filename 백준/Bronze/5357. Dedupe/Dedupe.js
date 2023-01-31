const fs = require('fs');
const [n, ...input] = fs.readFileSync('/dev/stdin').toString().split('\n');
let answer = '';
for (let i = 0; i < Number(n); i++) {
  const arr = [];
  for (let j = 0; j < input[i].length; j++) {
    if (arr.at(-1) !== input[i][j]) {
      arr.push(input[i][j]);
    }
  }
  answer += `${arr.join('')}\n`;
}
console.log(answer.trim());