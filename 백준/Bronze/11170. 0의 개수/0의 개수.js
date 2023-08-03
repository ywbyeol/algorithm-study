const f = r => {
  const [N, M] = r.split(' ');
  let c = 0;
  for (let i = +N; i <= +M; i++) c += [...`${i}`].filter(v => v === '0').length;
  return c;
};
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).map(f).join('\n'));