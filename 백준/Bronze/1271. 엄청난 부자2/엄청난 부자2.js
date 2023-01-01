const fs = require('fs');
const [N, M] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(BigInt);
console.log(`${N / M}\n${N % M}`);