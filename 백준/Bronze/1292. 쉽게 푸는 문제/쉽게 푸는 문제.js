const [S, A, B] = [[], ...`${require('fs').readFileSync(0)}`.trim().split(' ')];
for (let i = 1; i <= +B; i++) for (let j = 0; j < i; j++) S.push(i);
console.log(S.slice(+A - 1, +B).reduce((a, b) => a + +b, 0));