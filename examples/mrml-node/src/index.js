const fs = require("fs");
const path = require("path");
const { toHtml } = require("mrml/node/mrml");

if (process.argv.length < 3) {
  console.info("usage: npm start -- path/to/file.mjml");
  process.exit(1);
}

const filename = path.resolve(process.argv[2]);
const content = fs.readFileSync(filename).toString("utf8");

console.time("mrml");
try {
  console.log(toHtml(content));
  process.exit(0);
} catch (err) {
  console.error(err);
  process.exit(1);
}
