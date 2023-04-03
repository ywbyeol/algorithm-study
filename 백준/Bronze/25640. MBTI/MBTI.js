process.stdin.on('data', i => {
  const [J, ...F] = `${i}`.trim().split('\n');
  console.log(F.filter(x => x === J).length);
});