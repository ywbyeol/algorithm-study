const fs = require('fs');
const [T, input] = fs.readFileSync('/dev/stdin').toString().split('\n');
const inputArr = input.split(' ');
let answer = 0;
inputArr.forEach(el => {
  if (el === T) answer++;
});
console.log(answer);