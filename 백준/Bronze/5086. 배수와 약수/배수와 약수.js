let arr = `${require('fs').readFileSync(0)}`.trim().split('\n').slice(0, -1);
arr = arr.map(v => {
  const [a, b] = v.split(' ');
  return +b % +a === 0 ? 'factor' : +a % +b === 0 ? 'multiple' : 'neither';
});
console.log(arr.join('\n'));