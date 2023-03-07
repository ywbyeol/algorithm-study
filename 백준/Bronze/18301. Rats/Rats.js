process.stdin.on('data', i => {
  const [n1, n2, n12] = i.toString().trim().split(' ').map(Number);
  console.log(Math.floor(((n1 + 1) * (n2 + 1)) / (n12 + 1)) - 1);
});