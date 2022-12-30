const fs = require('fs');
const [input, setting] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n');
const inputArr = input.trim().split(' ');
let num1 =
  parseInt(inputArr[0]) * 60 + parseInt(inputArr[1]) + parseInt(setting);
if (num1 >= 1440) {
  num1 -= 1440;
}
const num2 = Math.floor(num1 / 60);
const num3 = num1 % 60;
console.log(`${num2} ${num3}`);