const fs = require('fs');
const [antenna, eyes] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split('\n')
  .map(Number);
let str = '';
if (antenna >= 3 && eyes <= 4) {
  str += 'TroyMartian\n';
}
if (antenna <= 6 && eyes >= 2) {
  str += 'VladSaturnian\n';
}
if (antenna <= 2 && eyes <= 3) {
  str += 'GraemeMercurian\n';
}
console.log(str.trim());