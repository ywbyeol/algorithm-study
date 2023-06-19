console.log(
  [...`${require('fs').readFileSync(0)}`.trim()]
    .filter((v, i, a) => v !== a[i - 1])
    .join(''),
);