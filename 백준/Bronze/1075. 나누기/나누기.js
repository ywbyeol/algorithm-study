const [N, F] = `${require('fs').readFileSync(0)}`.split('\n');
for (let i = 0; i < 100; i++)
  if ((+N.trim().slice(0, -2) * 100 + i) % +F === 0) {
    console.log(`${i}`.padStart(2, '0'));
    break;
  }