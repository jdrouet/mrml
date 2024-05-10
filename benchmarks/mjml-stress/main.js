if (process.argv.length < 4) {
  console.error('Invalid argument count.');
  console.error(`node main.js <count> <template-path>`);
  process.exit(1);
}

const count = +process.argv[2];
const templatePath = process.argv[3];

if (count <= 0) {
  console.error('Expected count greater that 0');
  process.exit(1);
}

const mjml = require('mjml');
const fs = require('fs');
const { performance } = require('perf_hooks');

console.log(`reading template ${templatePath}`);
const template = fs.readFileSync(templatePath, { encoding: 'utf8' }).toString();
console.log(`rendering ${count} times the template`);
console.log('starting...');
const start = performance.now();
for (let i = 0; i < count; i++) {
  const result = mjml(template);
}
const end = performance.now();
console.log('done!');
console.log(`executed in ${end - start}ms`);

