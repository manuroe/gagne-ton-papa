#!/usr/bin/env node
// Script to display QR code for the local HTTPS server URL

const qrcode = require('qrcode-terminal');
const os = require('os');

function getLocalIP() {
  const interfaces = os.networkInterfaces();
  for (const name of Object.keys(interfaces)) {
    for (const iface of interfaces[name]) {
      // Skip internal and non-IPv4 addresses
      if (iface.internal || iface.family !== 'IPv4') {
        continue;
      }
      return iface.address;
    }
  }
  return 'localhost';
}

const localIP = getLocalIP();
const port = process.env.PORT || 3000;
const url = `https://${localIP}:${port}`;

console.log('\n');
console.log('📱 Scan this QR code with your phone to access the app:');
console.log(`URL: ${url}`);
console.log('\n');
qrcode.generate(url, { small: true });
console.log('\n');
console.log('Note: You may need to accept the self-signed certificate on your phone.');
console.log('');
