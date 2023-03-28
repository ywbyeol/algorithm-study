const fs = require('fs');
const i = fs
  .readFileSync('/dev/stdin', 'utf8')
  .trim()
  .split(/\s+/)
  .slice(1)
  .map(Number);
const Y = i.reduce((a, b) => a + (b % 30 === 0 ? (b / 30 + 1) * 10 : Math.ceil(b / 30) * 10), 0);
const M = i.reduce((a, b) => a + (b % 60 === 0 ? (b / 60 + 1) * 15 : Math.ceil(b / 60) * 15), 0);
console.log(Y === M ? `Y M ${Y}` : `${Y < M ? 'Y' : 'M'} ${Y < M ? Y : M}`);