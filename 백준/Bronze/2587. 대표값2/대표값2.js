const i = `${require('fs').readFileSync(0)}`.trim().split('\n').map(Number);
console.log(`${i.reduce((a, b) => a + b) / 5}\n${i.sort((a, b) => a - b)[2]}`);