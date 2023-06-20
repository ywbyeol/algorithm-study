`${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .forEach(v =>
    console.log(Math.max(...(v.match(/(\d)\1+/g) || ['1']).map(x => x.length))),
  );