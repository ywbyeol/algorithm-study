const fs = require('fs');
const [UR, TR, UO, TO] = fs
  .readFileSync('/dev/stdin', 'utf8')
  .split(' ')
  .map(Number);
console.log(UR * 56 + TR * 24 + UO * 14 + TO * 6);