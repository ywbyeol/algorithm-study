process.stdin.on('data', i => {
  const [A, B] = i.toString().split('\n').map(Number);
  console.log(A === B ? 0 : A > B ? 1 : -1);
});