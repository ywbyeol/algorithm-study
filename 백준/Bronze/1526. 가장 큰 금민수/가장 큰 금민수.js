let N = +`${require('fs').readFileSync(0)}`.trim();
while (!/^[47]+$/.test(`${N}`)) N--;
console.log(N);