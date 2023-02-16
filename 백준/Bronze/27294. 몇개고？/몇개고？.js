const fs = require('fs');
const [T, S] = fs
  .readFileSync('dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
let rice = 0;
if (S === 1) {
  rice = 280;
} else if (T >= 12 && T <= 16) {
  rice = 320;
} else {
  rice = 280;
}
console.log(rice);