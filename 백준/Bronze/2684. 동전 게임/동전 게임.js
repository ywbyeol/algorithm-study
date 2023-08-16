const f = l =>
  ['TTT', 'TTH', 'THT', 'THH', 'HTT', 'HTH', 'HHT', 'HHH']
    .map(v => [l, l.slice(1), l.slice(2)].flatMap(x => x.match(/.{3}/g)).filter(w => w === v).length)
    .join(' ');
console.log(`${require('fs').readFileSync(0)}`.trim().split(/\s+/).slice(1).map(f).join('\n'));