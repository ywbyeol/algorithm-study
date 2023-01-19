const fs = require('fs');
const [N, M] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
let str = '';
if (M === 2 || M === 1) {
  str += 'NEWBIE!';
} else if (M <= N) {
  str += 'OLDBIE!';
} else {
  str += 'TLE!';
}
console.log(str);