const [T, ...I] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const ans = [];
for (let i = 0; i < +T; i++) {
  const [N, M] = I[i].split(' ');
  ans.push(`${+M * 2 - +N} ${+N - +M}`);
}
console.log(ans.join('\n'));