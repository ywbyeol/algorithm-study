`${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .slice(1)
  .forEach(v => {
    const sum = v
      .trim()
      .split('')
      .map((n, i) => (i % 2 === 0 ? (+n * 2 > 9 ? [...`${+n * 2}`].reduce((a, b) => a + +b, 0) : +n * 2) : +n))
      .reduce((a, b) => a + +b, 0);
    console.log(sum % 10 === 0 ? 'T' : 'F');
  });