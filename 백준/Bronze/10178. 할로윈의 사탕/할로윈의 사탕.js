const [T, ...I] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const ans = [];
for (let i = 0; i < +T; i++) {
  const [C, V] = I[i].split(' ');
  ans.push(`You get ${Math.floor(+C / +V)} piece(s) and your dad gets ${+C % +V} piece(s).`);
}
console.log(ans.join('\n'));