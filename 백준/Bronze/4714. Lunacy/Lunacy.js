process.stdin.on('data', i => {
  let answer = '';
  i.toString()
    .trim()
    .split('\n')
    .slice(0, -1)
    .map(Number)
    .forEach(
      n =>
        (answer += `Objects weighing ${n.toFixed(2)} on Earth will weigh ${(
          n * 0.167
        ).toFixed(2)} on the moon.\n`),
    );
  console.log(answer.trim());
});