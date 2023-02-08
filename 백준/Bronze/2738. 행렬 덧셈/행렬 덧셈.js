const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().trim().split('\n');
let [N, M] = input.shift().split(' ').map(Number);
const A = input.slice(0, N).map(n => n.split(' ').map(Number));
const B = input.slice(N).map(n => n.split(' ').map(Number));
const answer = [];
for (let i = 0; i < N; i++) {
  answer[i] = [];
  for (let j = 0; j < M; j++) {
    answer[i][j] = A[i][j] + B[i][j];
  }
}
console.log(answer.map(n => n.join(' ')).join('\n'));