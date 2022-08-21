const snaps = require("./snapshots/snapshots");
const fs = require("fs");

const RE = /.+ (.+):.+\d/;

for ([k, v] of Object.entries(snaps)) {
  const fileName = RE.exec(k)[1];
  fs.writeFileSync(`snapshots/${fileName}`, v);
}
