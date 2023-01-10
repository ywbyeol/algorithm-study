const fs = require('fs');
const [K, N, M] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
console.log(K * N - M >= 0 ? K * N - M : 0);