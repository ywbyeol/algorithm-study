const fs = require('fs');
const input = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
input.pop();
let answer = '';
for (let i = 0; i < input.length; i++) {
  answer += `Objects weighing ${input[i].toFixed(2)} on Earth will weigh ${(
    input[i] * 0.167
  ).toFixed(2)} on the moon.\n`;
}
console.log(answer.trim());