let I = `${require('fs').readFileSync(0)}`.trim().split(`\n`);
for (let i = 0; i < I.length; i++) if (I[i].includes('E')) I = I.slice(0, i);
console.log(I.map((v, i) => `Case ${i + 1}: ${eval(v)}`).join('\n'));