/usr/bin/hyperfine \
  --warmup 50 \
  "mjml $1" \
  "/usr/bin/mrml $1 render"
