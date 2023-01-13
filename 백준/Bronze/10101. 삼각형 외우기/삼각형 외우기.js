const fs = require('fs');
const [a, b, c] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let answer = '';
if (a + b + c === 180) {
  if (a === 60 && a === b && b === c) {
    answer = 'Equilateral';
  } else if (a !== b && b !== c && c !== a) {
    answer = 'Scalene';
  } else {
    answer = 'Isosceles';
  }
} else {
  answer = 'Error';
}
console.log(answer);