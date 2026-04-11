const mjml2html = require("mjml");
const pretty = require("pretty");
const fsp = require("fs/promises");

const COMPARE_DIRECTORY = "packages/mrml-core/resources/compare/success";
const TEMPLATE_DIRECTORY = "packages/mrml-core/resources/template";

const iterate = (list, callback) =>
  list.reduce((res, item) => res.then(() => callback(item)), Promise.resolve());

const cleanupIds = (content) => {
  console.log(`⌛️ doing some cleanup on ids`);
  // Collect all generated IDs referenced by for attributes (these are mjml-generated
  // for checkbox/label pairs in accordion, navbar, etc.)
  const generatedIds = new Set();
  for (const match of content.matchAll(/for="([a-zA-Z0-9]+)"/g)) {
    generatedIds.add(match[1]);
  }
  // Only normalize IDs that are part of a generated for/id pair
  for (const id of generatedIds) {
    content = content.replaceAll(`for="${id}"`, 'for="00000000"');
    content = content.replaceAll(`id="${id}"`, 'id="00000000"');
  }
  let matches;
  while (
    (matches = /mj-carousel-([a-zA-Z0-9]{12,16})-([a-zA-Z0-9\-]+)/g.exec(content))
  ) {
    const carouselId = matches[1];
    const extension = matches[2];
    content = content.replaceAll(
      `mj-carousel-${carouselId}-${extension}`,
      `mj-carousel-00000000-${extension}`,
    );
    content = content.replaceAll(
      `mj-carousel-radio-${carouselId}`,
      `mj-carousel-radio-00000000`,
    );
  }
  return content;
};

const cleanup = (content) => {
  console.log(`⌛️ doing some more cleanup`);
  return (
    content
      // empty style css blocks
      .replace(/<style\s+type="text\/css">\s*<\/style>/gim, "")
      .replace(/style=""/gim, "")
      // empty div blocks
      .replace(/<div\s*>\s*<\/div>/gim, "")
      .replace(/\s{2,}/gim, "\n")
      // percentages that are rounded in rust
      .replace(/33\.333333333333336/gm, "33.333332")
      .replace(/33-333333333333336/gm, "33-333332")
      // pretty formatter mangles @media screen, yahoo
      .replace(/@media screen yahoo/gm, "@media screen, yahoo")
      // pretty formatter self-closes non-void elements like canvas
      .replace(/<canvas\s*\/>/gim, "<canvas></canvas>")
  );
};

const fixMjmlInput = (content) => {
  // mjml 4.x has a camelCase bug: getMobileWidth checks
  // getAttribute('mobileWidth') !== 'mobileWidth', but the XML parser keeps
  // the value as-is ("mobile-width"). Rewrite to the camelCase form so that
  // the pixel-to-percent code path actually triggers.
  return content.replace(
    /mobile-width="mobile-width"/g,
    'mobileWidth="mobileWidth"',
  );
};

const handleFile = (dir, fname) => {
  console.log(`👉 starting ${dir}/${fname}`);
  return fsp
    .readFile(`${dir}/${fname}.mjml`, { encoding: "utf8" })
    .then((content) => {
      console.log(`⌛️ converting ${dir}/${fname}.mjml`);
      return mjml2html(fixMjmlInput(content)).html;
    })
    .then(pretty)
    .then(cleanupIds)
    .then(cleanup)
    .then(pretty)
    .then((output) => {
      console.log(`⌛️ writing ${dir}/${fname}.html file`);
      return fsp.writeFile(`${dir}/${fname}.html`, output, {
        encoding: "utf8",
      });
    })
    .then(() => console.log(`🎉 done with ${dir}/${fname}`))
    .catch((err) => console.error(`⛔️ error with ${fname}`, err));
};
const handleDirectory = (dir) => {
  console.log(`👉 starting with ${dir}`);
  return fsp
    .readdir(dir)
    .then((files) =>
      files
        .filter((fname) => fname.endsWith(".mjml"))
        .map((fname) => fname.substring(0, fname.length - 5)),
    )
    .then((files) => iterate(files, (fname) => handleFile(dir, fname)))
    .then(() => console.log(`🎉 done with ${dir}`));
};

iterate([COMPARE_DIRECTORY, TEMPLATE_DIRECTORY], handleDirectory).then(() =>
  console.log("🎉 All done"),
);
