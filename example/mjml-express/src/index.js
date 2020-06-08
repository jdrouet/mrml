const express = require("express");
const mjml2html = require("mjml");
const multer = require("multer");

const upload = multer({ storage: multer.memoryStorage() });

const app = express();

app.post("/render", upload.single("template"), (req, res) => {
  const template = req.file.buffer.toString('utf-8');
  res.send(mjml2html(template));
});

app.listen(process.env.PORT || 3000, (err) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }
  console.log('ready');
});
