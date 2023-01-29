const fs = require('fs');
const [A, P] = fs.readFileSync('/dev/stdin').toString().split(' ').map(Number);
const apple = A * 7;
const pale = P * 13;
if (apple === pale) {
  console.log('lika');
} else {
  console.log(apple > pale ? 'Axel' : 'Petra');
}