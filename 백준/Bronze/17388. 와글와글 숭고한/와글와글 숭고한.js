const fs = require('fs');
const [S, K, H] = fs
  .readFileSync('/dev/stdin')
  .toString()
  .trim()
  .split(' ')
  .map(Number);
const min = Math.min(S, K, H);
if (S + K + H >= 100) {
  console.log('OK');
  return;
}
if (min === S) {
  console.log('Soongsil');
  return;
}
if (min === K) {
  console.log('Korea');
  return;
}
if (min === H) {
  console.log('Hanyang');
  return;
}
console.log(min);