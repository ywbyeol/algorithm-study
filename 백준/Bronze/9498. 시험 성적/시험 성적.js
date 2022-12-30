const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim();
if (input < 60) {
  console.log('F');
  return;
}
if (input >= 90) {
  console.log('A');
  return;
}
if (input >= 80) {
  console.log('B');
  return;
}
if (input >= 70) {
  console.log('C');
  return
}
console.log('D');