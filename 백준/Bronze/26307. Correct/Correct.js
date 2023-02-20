const fs = require('fs');
const [HH, MM] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log((HH - 9) * 60 + MM);