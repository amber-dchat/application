const fs = require("fs");

const store_location = process.env.store_location;

console.log("🖊️ Installing Android Signature at", store_location);

let jks = process.env.jks;
let key = process.env.key;

jks = Buffer.from(jks, "base64");
key = Buffer.from(key, "base64");

fs.writeFileSync(store_location, jks);
fs.writeFileSync(
  "./src-tauri/gen/android/keystore.properties",
  ```${key}\nstoreFile=${store_location}\n```
);

console.log("✅ Android Signature Installed");
