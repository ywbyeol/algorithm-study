const fs = require('fs');
const [M, ...XY] = fs.readFileSync('/dev/stdin').toString().split('\n');
const cupArr = [1, 2, 3];
for (let i = 0; i < M; i++) {
  const [X, Y] = XY[i].split(' ').map(Number);
  const cup1 = cupArr.indexOf(X);
  const cup2 = cupArr.indexOf(Y);
  const temp = cupArr[cup1];
  cupArr[cup1] = cupArr[cup2];
  cupArr[cup2] = temp;
}
console.log(cupArr[0]);