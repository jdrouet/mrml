const fs = require("fs/promises");

const readFilesInDir = (name) =>
  fs
    .readdir(`pkg/${name}`)
    .then((list) => list.map((item) => `${name}/${item}`));

const listPackageFiles = () =>
  ["bundler", "nodejs", "web"].reduce(
    (previousPromise, item) =>
      previousPromise.then((previousContent) =>
        readFilesInDir(item).then((content) => [...previousContent, ...content])
      ),
    Promise.resolve([])
  );

const readPackageJson = () => fs.readFile("pkg/package.json").then(JSON.parse);

Promise.all([readPackageJson(), listPackageFiles()]).then(([pkg, files]) => {
  pkg.name = "mrml";
  pkg.files = files;

  pkg.main = 'nodejs/mrml_wasm.js';
  pkg.module = 'bundler/mrml_wasm.js';
  pkg.types = 'bundler/mrml_wasm.d.ts';

  return fs.writeFile("pkg/package.json", JSON.stringify(pkg, null, 2));
});
