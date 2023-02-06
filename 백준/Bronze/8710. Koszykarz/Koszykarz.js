const fs = require('fs');
const [k, w, m] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .split(' ')
  .map(Number);
console.log(Math.ceil((w - k) / m));